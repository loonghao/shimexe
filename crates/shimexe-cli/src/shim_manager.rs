use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

use shimexe_core::{ShimConfig, ShimRunner};

/// Manages shim files and operations
pub struct ShimManager {
    shim_dir: PathBuf,
}

impl ShimManager {
    /// Create a new shim manager
    pub fn new(custom_dir: Option<PathBuf>) -> Result<Self> {
        let shim_dir = if let Some(dir) = custom_dir {
            dir
        } else {
            dirs::home_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
                .join(".shimexe")
        };

        if !shim_dir.exists() {
            fs::create_dir_all(&shim_dir)?;
            info!("Created shim directory: {}", shim_dir.display());
        }

        Ok(Self { shim_dir })
    }

    /// Add a new shim
    pub fn add_shim(&self, name: &str, config: &ShimConfig) -> Result<()> {
        let shim_file = self.get_shim_file_path(name);
        config.to_file(&shim_file)?;

        // Create executable shim (copy of shimexe binary)
        self.create_executable_shim(name)?;

        debug!("Created shim file: {}", shim_file.display());
        Ok(())
    }

    /// Remove a shim
    pub fn remove_shim(&self, name: &str) -> Result<()> {
        let shim_file = self.get_shim_file_path(name);
        let exe_file = self.get_executable_path(name);

        if shim_file.exists() {
            fs::remove_file(&shim_file)?;
            debug!("Removed shim file: {}", shim_file.display());
        }

        if exe_file.exists() {
            fs::remove_file(&exe_file)?;
            debug!("Removed executable: {}", exe_file.display());
        }

        Ok(())
    }

    /// List all shims
    pub fn list_shims(&self) -> Result<Vec<(String, ShimConfig)>> {
        let mut shims = Vec::new();

        if !self.shim_dir.exists() {
            return Ok(shims);
        }

        for entry in fs::read_dir(&self.shim_dir)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.ends_with(".shim.toml") {
                    let name = file_name.trim_end_matches(".shim.toml");
                    if let Ok(config) = ShimConfig::from_file(&path) {
                        shims.push((name.to_string(), config));
                    }
                }
            }
        }

        Ok(shims)
    }

    /// Check if a shim exists
    pub fn shim_exists(&self, name: &str) -> bool {
        self.get_shim_file_path(name).exists()
    }

    /// Get shim configuration
    pub fn get_shim_config(&self, name: &str) -> Result<ShimConfig> {
        let shim_file = self.get_shim_file_path(name);
        Ok(ShimConfig::from_file(shim_file)?)
    }

    /// Update a shim
    pub fn update_shim(&self, name: &str, config: &ShimConfig) -> Result<()> {
        if !self.shim_exists(name) {
            return Err(anyhow::anyhow!("Shim '{}' does not exist", name));
        }

        self.add_shim(name, config)
    }

    /// Validate a shim
    pub fn validate_shim(&self, name: &str) -> Result<()> {
        let config = self.get_shim_config(name)?;
        let runner = ShimRunner::from_config(config)?;
        Ok(runner.validate()?)
    }

    /// Get the path to a shim file
    fn get_shim_file_path(&self, name: &str) -> PathBuf {
        self.shim_dir.join(format!("{}.shim.toml", name))
    }

    /// Get the path to the executable shim
    fn get_executable_path(&self, name: &str) -> PathBuf {
        let exe_ext = if cfg!(windows) { ".exe" } else { "" };
        self.shim_dir.join(format!("{}{}", name, exe_ext))
    }

    /// Create an executable shim by copying the current binary
    fn create_executable_shim(&self, name: &str) -> Result<()> {
        let current_exe = std::env::current_exe()?;
        let target_exe = self.get_executable_path(name);

        fs::copy(&current_exe, &target_exe)?;

        // On Unix-like systems, ensure the file is executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&target_exe)?.permissions();
            perms.set_mode(perms.mode() | 0o755);
            fs::set_permissions(&target_exe, perms)?;
        }

        debug!("Created executable shim: {}", target_exe.display());
        Ok(())
    }

    /// Get the shim directory
    pub fn shim_dir(&self) -> &Path {
        &self.shim_dir
    }

    /// Add shim directory to system PATH
    pub fn add_to_system_path(&self) -> Result<()> {
        self.add_directory_to_system_path(&self.shim_dir)
    }

    /// Add a specific directory to system PATH
    pub fn add_directory_to_system_path(&self, dir: &Path) -> Result<()> {
        let dir_str = dir.to_string_lossy();

        #[cfg(windows)]
        {
            self.add_to_windows_path(&dir_str)
        }

        #[cfg(unix)]
        {
            self.add_to_unix_path(&dir_str)
        }
    }

    #[cfg(windows)]
    fn add_to_windows_path(&self, dir: &str) -> Result<()> {
        use std::process::Command;

        // Check if directory is already in PATH
        if self.is_in_system_path(dir)? {
            info!("Directory already in system PATH: {}", dir);
            return Ok(());
        }

        // Add to system PATH using PowerShell
        let script = format!(
            r#"
            $currentPath = [Environment]::GetEnvironmentVariable('PATH', 'Machine')
            $newPath = $currentPath + ';{}'
            [Environment]::SetEnvironmentVariable('PATH', $newPath, 'Machine')
            "#,
            dir
        );

        let output = Command::new("powershell")
            .args(["-Command", &script])
            .output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to add to system PATH: {}", error));
        }

        info!("Added directory to system PATH: {}", dir);
        println!("✓ Added {} to system PATH", dir);
        println!("  Note: You may need to restart your terminal for changes to take effect");

        Ok(())
    }

    #[cfg(unix)]
    fn add_to_unix_path(&self, dir: &str) -> Result<()> {
        use std::fs::OpenOptions;
        use std::io::Write;

        // Check if directory is already in PATH
        if self.is_in_system_path(dir)? {
            info!("Directory already in system PATH: {}", dir);
            return Ok(());
        }

        // Add to shell profile files
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

        let profile_files = [
            home.join(".bashrc"),
            home.join(".zshrc"),
            home.join(".profile"),
        ];

        let export_line = format!("export PATH=\"{}:$PATH\"\n", dir);

        for profile_file in &profile_files {
            if profile_file.exists() {
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(profile_file)?;

                writeln!(file, "\n# Added by shimexe")?;
                write!(file, "{}", export_line)?;

                info!("Added PATH export to: {}", profile_file.display());
            }
        }

        info!("Added directory to shell profiles: {}", dir);
        println!("✓ Added {} to shell profiles", dir);
        println!(
            "  Note: Run 'source ~/.bashrc' or restart your terminal for changes to take effect"
        );

        Ok(())
    }

    /// Check if a directory is already in system PATH
    fn is_in_system_path(&self, dir: &str) -> Result<bool> {
        if let Ok(path_var) = std::env::var("PATH") {
            let separator = if cfg!(windows) { ';' } else { ':' };
            for path_entry in path_var.split(separator) {
                if path_entry.trim() == dir {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
