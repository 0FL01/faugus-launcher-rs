// Wine tools module
// Handles running winecfg and winetricks

use crate::config::paths::Paths;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::info;

/// Run winetricks for a specific prefix
pub fn run_winetricks(prefix: &Path, runner: &str) -> Result<()> {
    info!("Running Winetricks for prefix: {:?}", prefix);

    let umu_run = get_umu_run()?;
    let mut cmd = Command::new(&umu_run);

    cmd.env("WINEPREFIX", prefix);
    cmd.env("GAMEID", "winetricks-gui");
    cmd.env("STORE", "none");

    let runner_path =
        if runner.is_empty() || runner == "UMU-Proton Latest" || runner.contains("(default)") {
            String::new()
        } else {
            runner.to_string()
        };

    if !runner_path.is_empty() {
        cmd.env("PROTONPATH", &runner_path);
    }

    // umu-run expects at least one argument for the executable
    cmd.arg("");

    cmd.spawn().with_context(|| "Failed to run Winetricks")?;

    Ok(())
}

/// Run winecfg for a specific prefix
pub fn run_winecfg(prefix: &Path, runner: &str, game_id: Option<&str>) -> Result<()> {
    info!("Running Winecfg for prefix: {:?}", prefix);

    let umu_run = get_umu_run()?;
    let mut cmd = Command::new(&umu_run);

    cmd.env("WINEPREFIX", prefix);
    cmd.env("GAMEID", game_id.unwrap_or("default"));

    let runner_path =
        if runner.is_empty() || runner == "UMU-Proton Latest" || runner.contains("(default)") {
            String::new()
        } else {
            runner.to_string()
        };

    if !runner_path.is_empty() {
        cmd.env("PROTONPATH", &runner_path);
    }

    cmd.arg("winecfg");

    cmd.spawn().with_context(|| "Failed to run Winecfg")?;

    Ok(())
}

/// Get umu-run binary path
fn get_umu_run() -> Result<PathBuf> {
    let umu_run = Paths::umu_run();

    if !umu_run.exists() {
        if let Some(path) = Paths::find_binary("umu-run") {
            return Ok(path);
        }

        return Err(anyhow::anyhow!(
            "umu-run not found. Please install UMU-Launcher."
        ));
    }

    Ok(umu_run)
}
