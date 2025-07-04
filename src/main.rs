use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use std::env;
use std::path::PathBuf;

mod commands;
mod shim_manager;

use commands::*;
use shimexe_core::prelude::*;

#[derive(Parser)]
#[command(name = "shimexe")]
#[command(about = "A modern, cross-platform executable shim manager")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Shim directory (defaults to ~/.shimexe/shims)
    #[arg(long, global = true)]
    shim_dir: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new shim
    Add(AddCommand),
    /// Remove a shim
    Remove(RemoveCommand),
    /// List all shims
    List(ListCommand),
    /// Update an existing shim
    Update(UpdateCommand),
    /// Validate a shim configuration
    Validate(ValidateCommand),
    /// Run a shim (used internally when shimexe is copied/linked as the target executable)
    Run(RunCommand),
    /// Initialize shimexe configuration
    Init(InitCommand),
    /// Check for updates
    CheckUpdate(UpdateCheckCommand),
    /// Manage auto-update settings
    AutoUpdate(AutoUpdateCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    // Determine if we're running as a shim BEFORE parsing CLI args
    let current_exe = env::current_exe()?;
    let exe_name = current_exe
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("shimexe");

    // If the executable name is not "shimexe", we're likely running as a shim
    if exe_name != "shimexe" {
        // Initialize minimal logging for shim mode (only errors by default)
        tracing_subscriber::fmt()
            .with_env_filter("shimexe=error")
            .init();

        return run_as_shim(exe_name, &env::args().collect::<Vec<_>>()[1..]);
    }

    // Parse CLI only if we're running as the main shimexe binary
    let cli = Cli::parse();

    // Initialize logging - default to warn level for external crates, verbose enables debug for all
    let (shimexe_level, turbo_cdn_level) = if cli.verbose {
        ("debug", "debug")
    } else {
        ("info", "warn")
    };
    tracing_subscriber::fmt()
        .with_env_filter(format!("shimexe={},turbo_cdn={}", shimexe_level, turbo_cdn_level))
        .init();

    // Run as main CLI
    match cli.command {
        Some(Commands::Add(cmd)) => cmd.execute(cli.shim_dir).await,
        Some(Commands::Remove(cmd)) => cmd.execute(cli.shim_dir),
        Some(Commands::List(cmd)) => cmd.execute(cli.shim_dir),
        Some(Commands::Update(cmd)) => cmd.execute(cli.shim_dir),
        Some(Commands::Validate(cmd)) => cmd.execute(),
        Some(Commands::Run(cmd)) => cmd.execute(cli.shim_dir),
        Some(Commands::Init(cmd)) => cmd.execute(cli.shim_dir),
        Some(Commands::CheckUpdate(cmd)) => cmd.execute(cli.shim_dir),
        Some(Commands::AutoUpdate(cmd)) => cmd.execute(cli.shim_dir),
        None => {
            // No command provided, show help
            let mut cmd = Cli::command();
            cmd.print_help()?;
            Ok(())
        }
    }
}

/// Run the executable as a shim
fn run_as_shim(shim_name: &str, args: &[String]) -> Result<()> {
    // First, try to find the shim file in the same directory as the executable
    let current_exe = env::current_exe()?;
    let exe_dir = current_exe
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    let local_shim_file = exe_dir.join(format!("{}.shim.toml", shim_name));

    let shim_file = if local_shim_file.exists() {
        local_shim_file
    } else {
        // Fallback to the default shim directory
        let shim_dir = get_shim_directory(None)?;
        let default_shim_file = shim_dir.join(format!("{}.shim.toml", shim_name));

        if !default_shim_file.exists() {
            return Err(anyhow::anyhow!(
                "Shim '{}' not found. Searched in:\n  - {}\n  - {}",
                shim_name,
                local_shim_file.display(),
                default_shim_file.display()
            ));
        }

        default_shim_file
    };

    let runner = ShimRunner::from_file(&shim_file)?;
    let exit_code = runner.execute(args)?;

    std::process::exit(exit_code);
}

/// Get the shim directory, creating it if it doesn't exist
fn get_shim_directory(custom_dir: Option<PathBuf>) -> Result<PathBuf> {
    let shim_dir = if let Some(dir) = custom_dir {
        dir
    } else {
        dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
            .join(".shimexe")
    };

    if !shim_dir.exists() {
        std::fs::create_dir_all(&shim_dir)?;
    }

    Ok(shim_dir)
}
