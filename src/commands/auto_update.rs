use anyhow::Result;
use clap::{Args, Subcommand};
use std::path::PathBuf;
use tracing::info;

use crate::shim_manager::ShimManager;
use shimexe_core::{AutoUpdate, UpdateProvider, VersionCheck};

#[derive(Args)]
pub struct AutoUpdateCommand {
    #[command(subcommand)]
    pub action: AutoUpdateAction,
}

#[derive(Subcommand)]
pub enum AutoUpdateAction {
    /// Enable auto-update for a shim
    Enable(EnableAutoUpdateCommand),
    /// Disable auto-update for a shim
    Disable(DisableAutoUpdateCommand),
    /// Configure auto-update settings
    Configure(ConfigureAutoUpdateCommand),
    /// Show auto-update status
    Status(StatusAutoUpdateCommand),
}

#[derive(Args)]
pub struct EnableAutoUpdateCommand {
    /// Name of the shim
    pub name: String,

    /// Update provider type (github, https, custom)
    #[arg(short, long, default_value = "github")]
    pub provider: String,

    /// Repository for GitHub provider (owner/repo)
    #[arg(long)]
    pub repo: Option<String>,

    /// Asset pattern for GitHub provider
    #[arg(long)]
    pub asset_pattern: Option<String>,

    /// Check interval in hours
    #[arg(long, default_value = "24")]
    pub interval: u64,
}

#[derive(Args)]
pub struct DisableAutoUpdateCommand {
    /// Name of the shim
    pub name: String,
}

#[derive(Args)]
pub struct ConfigureAutoUpdateCommand {
    /// Name of the shim
    pub name: String,

    /// Check interval in hours
    #[arg(long)]
    pub interval: Option<u64>,

    /// Enable/disable pre-releases for GitHub provider
    #[arg(long)]
    pub prerelease: Option<bool>,

    /// Pre-update command
    #[arg(long)]
    pub pre_command: Option<String>,

    /// Post-update command
    #[arg(long)]
    pub post_command: Option<String>,
}

#[derive(Args)]
pub struct StatusAutoUpdateCommand {
    /// Name of the shim (optional, shows all if not specified)
    pub name: Option<String>,
}

impl AutoUpdateCommand {
    pub fn execute(&self, shim_dir: Option<PathBuf>) -> Result<()> {
        match &self.action {
            AutoUpdateAction::Enable(cmd) => cmd.execute(shim_dir),
            AutoUpdateAction::Disable(cmd) => cmd.execute(shim_dir),
            AutoUpdateAction::Configure(cmd) => cmd.execute(shim_dir),
            AutoUpdateAction::Status(cmd) => cmd.execute(shim_dir),
        }
    }
}

impl EnableAutoUpdateCommand {
    pub fn execute(&self, shim_dir: Option<PathBuf>) -> Result<()> {
        let manager = ShimManager::new(shim_dir)?;

        if !manager.shim_exists(&self.name) {
            return Err(anyhow::anyhow!("Shim '{}' does not exist", self.name));
        }

        let mut config = manager.get_shim_config(&self.name)?;

        let auto_update = match self.provider.as_str() {
            "github" => {
                let repo = self
                    .repo
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("--repo is required for GitHub provider"))?;
                let asset_pattern = self.asset_pattern.as_ref().ok_or_else(|| {
                    anyhow::anyhow!("--asset-pattern is required for GitHub provider")
                })?;

                AutoUpdate {
                    enabled: true,
                    provider: UpdateProvider::Github {
                        repo: repo.clone(),
                        asset_pattern: asset_pattern.clone(),
                        include_prerelease: false,
                    },
                    download_url: format!(
                        "https://github.com/{}/releases/download/{{version}}/{}",
                        repo, asset_pattern
                    ),
                    version_check: VersionCheck::GithubLatest {
                        repo: repo.clone(),
                        include_prerelease: false,
                    },
                    check_interval_hours: self.interval,
                    pre_update_command: None,
                    post_update_command: None,
                }
            }
            "https" => {
                return Err(anyhow::anyhow!(
                    "HTTPS provider configuration not yet implemented"
                ));
            }
            "custom" => {
                return Err(anyhow::anyhow!(
                    "Custom provider configuration not yet implemented"
                ));
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown provider type: {}", self.provider));
            }
        };

        config.auto_update = Some(auto_update);
        manager.update_shim(&self.name, &config)?;

        info!("Enabled auto-update for shim '{}'", self.name);
        println!("✅ Auto-update enabled for '{}'", self.name);
        println!("   Provider: {}", self.provider);
        println!("   Check interval: {} hours", self.interval);

        Ok(())
    }
}

impl DisableAutoUpdateCommand {
    pub fn execute(&self, shim_dir: Option<PathBuf>) -> Result<()> {
        let manager = ShimManager::new(shim_dir)?;

        if !manager.shim_exists(&self.name) {
            return Err(anyhow::anyhow!("Shim '{}' does not exist", self.name));
        }

        let mut config = manager.get_shim_config(&self.name)?;

        if config.auto_update.is_none() {
            println!("Auto-update is already disabled for '{}'", self.name);
            return Ok(());
        }

        config.auto_update = None;
        manager.update_shim(&self.name, &config)?;

        info!("Disabled auto-update for shim '{}'", self.name);
        println!("✅ Auto-update disabled for '{}'", self.name);

        Ok(())
    }
}

impl ConfigureAutoUpdateCommand {
    pub fn execute(&self, shim_dir: Option<PathBuf>) -> Result<()> {
        let manager = ShimManager::new(shim_dir)?;

        if !manager.shim_exists(&self.name) {
            return Err(anyhow::anyhow!("Shim '{}' does not exist", self.name));
        }

        let mut config = manager.get_shim_config(&self.name)?;

        let mut auto_update = config
            .auto_update
            .ok_or_else(|| anyhow::anyhow!("Auto-update is not enabled for '{}'", self.name))?;

        // Update interval if specified
        if let Some(interval) = self.interval {
            auto_update.check_interval_hours = interval;
            println!("Updated check interval to {} hours", interval);
        }

        // Update prerelease setting for GitHub provider
        if let Some(prerelease) = self.prerelease {
            match &mut auto_update.provider {
                UpdateProvider::Github {
                    include_prerelease, ..
                } => {
                    *include_prerelease = prerelease;
                    println!("Updated prerelease setting to {}", prerelease);
                }
                _ => {
                    println!("Warning: --prerelease only applies to GitHub provider");
                }
            }

            if let VersionCheck::GithubLatest {
                include_prerelease, ..
            } = &mut auto_update.version_check
            {
                *include_prerelease = prerelease;
            }
        }

        // Update commands
        if let Some(ref pre_command) = self.pre_command {
            auto_update.pre_update_command = Some(pre_command.clone());
            println!("Updated pre-update command");
        }

        if let Some(ref post_command) = self.post_command {
            auto_update.post_update_command = Some(post_command.clone());
            println!("Updated post-update command");
        }

        config.auto_update = Some(auto_update);
        manager.update_shim(&self.name, &config)?;

        info!("Updated auto-update configuration for shim '{}'", self.name);
        println!("✅ Auto-update configuration updated for '{}'", self.name);

        Ok(())
    }
}

impl StatusAutoUpdateCommand {
    pub fn execute(&self, shim_dir: Option<PathBuf>) -> Result<()> {
        let manager = ShimManager::new(shim_dir)?;

        if let Some(ref name) = self.name {
            self.show_single_status(&manager, name)?;
        } else {
            self.show_all_status(&manager)?;
        }

        Ok(())
    }

    fn show_single_status(&self, manager: &ShimManager, name: &str) -> Result<()> {
        if !manager.shim_exists(name) {
            return Err(anyhow::anyhow!("Shim '{}' does not exist", name));
        }

        let config = manager.get_shim_config(name)?;

        println!("📦 {}", name);

        if let Some(ref auto_update) = config.auto_update {
            println!("   Status: ✅ Enabled");
            println!(
                "   Provider: {}",
                self.format_provider(&auto_update.provider)
            );
            println!(
                "   Check interval: {} hours",
                auto_update.check_interval_hours
            );

            if let Some(ref cmd) = auto_update.pre_update_command {
                println!("   Pre-update command: {}", cmd);
            }

            if let Some(ref cmd) = auto_update.post_update_command {
                println!("   Post-update command: {}", cmd);
            }
        } else {
            println!("   Status: ❌ Disabled");
        }

        Ok(())
    }

    fn show_all_status(&self, manager: &ShimManager) -> Result<()> {
        let shims = manager.list_shims()?;

        if shims.is_empty() {
            println!("No shims found.");
            return Ok(());
        }

        println!("Auto-update status for all shims:\n");

        for (name, config) in shims {
            println!("📦 {}", name);

            if let Some(ref auto_update) = config.auto_update {
                println!("   Status: ✅ Enabled");
                println!(
                    "   Provider: {}",
                    self.format_provider(&auto_update.provider)
                );
                println!("   Interval: {} hours", auto_update.check_interval_hours);
            } else {
                println!("   Status: ❌ Disabled");
            }

            println!();
        }

        Ok(())
    }

    fn format_provider(&self, provider: &UpdateProvider) -> String {
        match provider {
            UpdateProvider::Github { repo, .. } => format!("GitHub ({})", repo),
            UpdateProvider::Https { base_url, .. } => format!("HTTPS ({})", base_url),
            UpdateProvider::Custom { .. } => "Custom".to_string(),
        }
    }
}
