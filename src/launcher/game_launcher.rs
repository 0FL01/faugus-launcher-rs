// Game launcher module
// Handles launching games with UMU-Launcher and Proton

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use sysinfo::{Pid, System};
use tokio::process::Command as AsyncCommand;
use tracing::info;

use crate::config::paths::Paths;
use crate::config::Game;
use crate::proton::runner_resolver;

/// Process information for running games
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameProcess {
    pub game_title: String,
    pub main_pid: u32,
    pub umu_pid: Option<u32>,
}

/// Game launcher
pub struct GameLauncher;

impl GameLauncher {
    /// Launch a game
    pub async fn launch(game: &Game) -> Result<GameProcess> {
        info!("Launching game: {}", game.title);

        // Ensure required directories exist
        Self::ensure_directories(game)?;

        // Build the command
        let umu_run = Self::get_umu_run()?;
        let mut cmd = AsyncCommand::new(&umu_run);

        // Set environment variables
        Self::setup_environment(&mut cmd, game)?;

        // Set up arguments
        let args = Self::build_arguments(game);
        cmd.args(&args);

        // Spawn the process
        let child = cmd
            .spawn()
            .with_context(|| format!("Failed to launch game: {}", game.title))?;

        let pid = child.id().unwrap_or(0);
        info!("Game {} launched with PID: {}", game.title, pid);

        Ok(GameProcess {
            game_title: game.title.clone(),
            main_pid: pid,
            umu_pid: None,
        })
    }

    /// Ensure required directories exist
    fn ensure_directories(game: &Game) -> Result<()> {
        // Create prefix if it doesn't exist
        if !game.prefix.exists() {
            std::fs::create_dir_all(&game.prefix)
                .with_context(|| format!("Failed to create prefix: {:?}", game.prefix))?;
        }

        // Create logs directory
        let logs_dir = Paths::logs_dir();
        if !logs_dir.exists() {
            std::fs::create_dir_all(&logs_dir)
                .with_context(|| format!("Failed to create logs directory: {:?}", logs_dir))?;
        }

        Ok(())
    }

    /// Get umu-run binary
    fn get_umu_run() -> Result<PathBuf> {
        let umu_run = Paths::umu_run();

        if !umu_run.exists() {
            // Try to find in PATH
            if let Some(path) = Paths::find_binary("umu-run") {
                return Ok(path);
            }

            return Err(anyhow::anyhow!(
                "umu-run not found. Please install UMU-Launcher."
            ));
        }

        Ok(umu_run)
    }

    /// Set up environment variables for the game
    fn setup_environment(cmd: &mut tokio::process::Command, game: &Game) -> Result<()> {
        // Wine prefix
        cmd.env("WINEPREFIX", &game.prefix);

        // Proton runner
        runner_resolver::validate_runner(&game.runner)?;
        let runner = runner_resolver::resolve_runner(&game.runner)?;

        if !runner.is_empty() {
            cmd.env("PROTONPATH", &runner);
        }

        // Game ID for UMU
        cmd.env("GAMEID", &game.gameid);

        // MangoHud
        if game.mangohud {
            if let Some(_mangohud) = Paths::mangohud() {
                info!("Enabling MangoHud");
                cmd.env("MANGOHUD", "1");
            }
        }

        // GameMode
        if game.gamemode {
            if let Some(gamemoderun) = Paths::gamemoderun() {
                info!("Enabling GameMode");
                cmd.env("LD_PRELOAD", gamemoderun.to_string_lossy().to_string());
            }
        }

        // Disable hidraw
        if game.disable_hidraw {
            cmd.env("WINE_DISABLE_DISABLE_HIDRAW", "1");
        }

        // Wayland driver
        if let Ok(config) = std::fs::read_to_string(Paths::config_file()) {
            if config.contains("wayland-driver=true") {
                cmd.env("PROTON_USE_WINE_DXGI", "1");
            }
        }

        // HDR
        if let Ok(config) = std::fs::read_to_string(Paths::config_file()) {
            if config.contains("enable-hdr=true") {
                cmd.env("ENABLE_HDR", "1");
            }
        }

        // WOW64
        if let Ok(config) = std::fs::read_to_string(Paths::config_file()) {
            if config.contains("enable-wow64=true") {
                cmd.env("WINEARCH", "win64");
            }
        }

        // Lossless scaling
        if game.lossless_enabled {
            if let Some(lossless_dll) = Paths::find_lossless_dll() {
                info!("Enabling Lossless Scaling");
                cmd.env("WINEDLLOVERRIDES", "dxgi=n");
                cmd.env("LD_PRELOAD", lossless_dll.to_string_lossy().to_string());
            }
        }

        // discrete GPU
        if let Ok(config) = std::fs::read_to_string(Paths::config_file()) {
            if config.contains("discrete-gpu=true") {
                cmd.env("__GLX_VENDOR_LIBRARY_NAME", "nvidia");
            }
        }

        // Proton fixes
        if !game.protonfix.is_empty() {
            cmd.env("PROTON_NO_FSYNC", "1");
            cmd.env("PROTON_NO_ESYNC", "1");
        }

        // Logging
        if let Ok(config) = std::fs::read_to_string(Paths::config_file()) {
            if config.contains("enable-logging=true") {
                let _log_file = Paths::logs_dir().join(format!("{}.log", game.gameid));
                cmd.env("WINEDEBUG", "+all");
                cmd.env("WINE_MONO_TRACE", "E:System.Windows.Forms");
            }
        }

        Ok(())
    }

    /// Build command arguments
    fn build_arguments(game: &Game) -> Vec<String> {
        let mut args = Vec::new();

        // Game executable path
        args.push(game.path.to_string_lossy().to_string());

        // Launch arguments (e.g., -no-dwrite)
        if !game.launch_arguments.is_empty() {
            for arg in game.launch_arguments.split_whitespace() {
                args.push(arg.to_string());
            }
        }

        // Game-specific arguments
        if !game.game_arguments.is_empty() {
            for arg in game.game_arguments.split_whitespace() {
                args.push(arg.to_string());
            }
        }

        args
    }

    /// Check if a process is running
    pub fn is_process_running(pid: u32) -> bool {
        let mut sys = System::new();
        sys.refresh_processes();

        let pid = Pid::from_u32(pid);
        sys.processes().contains_key(&pid)
    }

    /// Terminate a game process
    pub fn terminate(pid: u32) -> Result<()> {
        info!("Terminating process: {}", pid);

        #[cfg(unix)]
        {
            use nix::sys::signal::{self, Signal};
            use nix::unistd::Pid;
            signal::kill(Pid::from_raw(pid as i32), Signal::SIGTERM)
                .with_context(|| format!("Failed to send SIGTERM to process {}", pid))?;
        }

        #[cfg(windows)]
        {
            Command::new("taskkill")
                .args(["/PID", &pid.to_string(), "/F"])
                .output()
                .with_context(|| format!("Failed to kill process {}", pid))?;
        }

        Ok(())
    }

    /// Get game process by title
    pub fn get_game_process(title: &str) -> Option<GameProcess> {
        let running_games = Paths::running_games_json();

        if !running_games.exists() {
            return None;
        }

        let content = std::fs::read_to_string(running_games).ok()?;
        let processes: Vec<GameProcess> = serde_json::from_str(&content).ok()?;

        processes.into_iter().find(|p| p.game_title == title)
    }

    /// Save running game process
    pub fn save_process(process: &GameProcess) -> Result<()> {
        let running_games = Paths::running_games_json();

        let mut processes: Vec<GameProcess> = if running_games.exists() {
            let content = std::fs::read_to_string(&running_games)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Vec::new()
        };

        processes.push(process.clone());

        let content = serde_json::to_string_pretty(&processes)?;
        std::fs::write(&running_games, content)?;

        Ok(())
    }

    /// Remove game process from running games
    pub fn remove_process(title: &str) -> Result<()> {
        let running_games = Paths::running_games_json();

        if !running_games.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(&running_games)?;
        let mut processes: Vec<GameProcess> = serde_json::from_str(&content)?;

        processes.retain(|p| p.game_title != title);

        let content = serde_json::to_string_pretty(&processes)?;
        std::fs::write(&running_games, content)?;

        Ok(())
    }

    /// Update latest games file
    pub fn update_latest_games(title: &str) -> Result<()> {
        let latest_games = Paths::latest_games_txt();

        let content = if latest_games.exists() {
            std::fs::read_to_string(&latest_games).unwrap_or_default()
        } else {
            String::new()
        };

        let mut games: Vec<String> = content
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        // Remove if already exists
        games.retain(|g| g != title);

        // Add to front
        games.insert(0, title.to_string());

        // Keep only last 10
        games.truncate(10);

        std::fs::write(&latest_games, games.join("\n") + "\n")?;

        Ok(())
    }
}
