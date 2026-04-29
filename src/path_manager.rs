use anyhow::Result;
use std::path::Path;
use tracing::info;

pub trait SystemPathManager: Send + Sync {
    fn add_directory_to_system_path(&self, dir: &Path) -> Result<()>;
}

pub struct DefaultSystemPathManager;

impl SystemPathManager for DefaultSystemPathManager {
    fn add_directory_to_system_path(&self, dir: &Path) -> Result<()> {
        let dir_str = dir.to_string_lossy();

        #[cfg(windows)]
        {
            add_to_windows_path(&dir_str)
        }

        #[cfg(unix)]
        {
            add_to_unix_path(&dir_str)
        }
    }
}

#[cfg(windows)]
fn add_to_windows_path(dir: &str) -> Result<()> {
    use std::process::Command;

    if is_in_system_path(dir)? {
        info!("Directory already in system PATH: {}", dir);
        return Ok(());
    }

    let script = format!(
        r#"
            $currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
            $newPath = $currentPath + ';{}'
            [Environment]::SetEnvironmentVariable('PATH', $newPath, 'User')
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
    println!("Added {} to user PATH", dir);
    println!("  Note: You may need to restart your terminal for changes to take effect");

    Ok(())
}

#[cfg(unix)]
fn add_to_unix_path(dir: &str) -> Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    if is_in_system_path(dir)? {
        info!("Directory already in system PATH: {}", dir);
        return Ok(());
    }

    let home =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

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
    println!("Added {} to shell profiles", dir);
    println!("  Note: Run 'source ~/.bashrc' or restart your terminal for changes to take effect");

    Ok(())
}

#[cfg(windows)]
fn is_in_system_path(dir: &str) -> Result<bool> {
    use std::process::Command;

    let script = format!(
        r#"
        $userPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
        $userPath.Split(';') | ForEach-Object {{ $_.Trim() }} | Where-Object {{ $_ -eq '{}' }}
        "#,
        dir
    );

    let output = Command::new("powershell")
        .args(["-Command", &script])
        .output()?;

    if !output.status.success() {
        // Fallback to checking current process PATH
        return check_process_path(dir);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(!stdout.trim().is_empty())
}

#[cfg(unix)]
fn is_in_system_path(dir: &str) -> Result<bool> {
    check_process_path(dir)
}

fn check_process_path(dir: &str) -> Result<bool> {
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
