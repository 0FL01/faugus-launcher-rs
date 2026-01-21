// Game launch controller
// Manages game launching, process monitoring, and UI state

use iced::Task;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{error, info};

use crate::config::Game;
use crate::launcher::game_launcher::{GameLauncher, GameProcess};

/// Launch status for a game
#[derive(Debug, Clone, PartialEq)]
pub enum LaunchStatus {
    NotRunning,
    Launching,
    Running(GameProcess),
    Error(String),
}

/// Game launch controller
#[derive(Debug, Clone)]
pub struct GameLaunchController {
    /// Currently running games
    running_games: Arc<Mutex<HashMap<String, LaunchStatus>>>,
}

impl GameLaunchController {
    /// Create a new launch controller
    pub fn new() -> Self {
        Self {
            running_games: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get the launch status of a game
    pub fn get_status(&self, title: &str) -> LaunchStatus {
        let games = self.running_games.lock().unwrap_or_else(|e| e.into_inner());
        games
            .get(title)
            .cloned()
            .unwrap_or(LaunchStatus::NotRunning)
    }

    /// Check if a game is currently running
    /// TODO: Use for UI status indicators, double-launch prevention
    #[allow(dead_code)]
    pub fn is_running(&self, title: &str) -> bool {
        matches!(
            self.get_status(title),
            LaunchStatus::Running(_) | LaunchStatus::Launching
        )
    }

    /// Launch a game
    pub fn launch_game(&self, game: Game) -> Task<LaunchMessage> {
        info!("Preparing to launch game: {}", game.title);

        let title = game.title.clone();
        let running_games = self.running_games.clone();

        // Set status to launching
        {
            let mut games = running_games.lock().unwrap_or_else(|e| e.into_inner());
            games.insert(title.clone(), LaunchStatus::Launching);
        }

        // Spawn async task to launch the game
        Task::perform(
            async move {
                // Launch the game
                match GameLauncher::launch(&game).await {
                    Ok(process) => {
                        info!("Game {} launched with PID: {}", title, process.main_pid);

                        // Save process info
                        if let Err(e) = GameLauncher::save_process(&process) {
                            error!("Failed to save process info: {}", e);
                        }

                        // Update latest games
                        if let Err(e) = GameLauncher::update_latest_games(&title) {
                            error!("Failed to update latest games: {}", e);
                        }

                        // Update status
                        {
                            let mut games = running_games.lock().unwrap_or_else(|e| e.into_inner());
                            games.insert(title.clone(), LaunchStatus::Running(process.clone()));
                        }

                        LaunchMessage::Launched(title.clone(), process)
                    }
                    Err(e) => {
                        error!("Failed to launch game {}: {}", title, e);

                        // Update status
                        {
                            let mut games = running_games.lock().unwrap_or_else(|e| e.into_inner());
                            games.insert(title.clone(), LaunchStatus::Error(e.to_string()));
                        }

                        LaunchMessage::LaunchFailed(title.clone(), e.to_string())
                    }
                }
            },
            |msg| msg,
        )
    }

    /// Terminate a running game
    pub fn terminate_game(&self, title: &str) -> Result<(), String> {
        let status = self.get_status(title);

        match status {
            LaunchStatus::Running(process) => {
                info!("Terminating game: {}", title);

                // Terminate the main process
                GameLauncher::terminate(process.main_pid)
                    .map_err(|e| format!("Failed to terminate game: {}", e))?;

                // Remove from running games
                {
                    let mut games = self.running_games.lock().unwrap_or_else(|e| e.into_inner());
                    games.remove(title);
                }

                // Remove process info
                let _ = GameLauncher::remove_process(title);

                Ok(())
            }
            LaunchStatus::Launching => Err("Game is still launching, please wait".to_string()),
            LaunchStatus::NotRunning => Err("Game is not running".to_string()),
            LaunchStatus::Error(_) => Err("Game launch failed".to_string()),
        }
    }

    /// Terminate all running games
    pub fn terminate_all(&self) {
        let running = self.get_running_games();
        for (title, _) in running {
            let _ = self.terminate_game(&title);
        }

        // Force kill any remaining wine processes
        GameLauncher::kill_all_wine_processes();
    }

    /// Update game status (call when process exits)
    pub fn on_process_exited(&self, title: &str) {
        info!("Game process exited: {}", title);

        // Remove from running games
        {
            let mut games = self.running_games.lock().unwrap_or_else(|e| e.into_inner());
            games.remove(title);
        }

        // Remove process info
        let _ = GameLauncher::remove_process(title);
    }

    /// Get all running games
    pub fn get_running_games(&self) -> Vec<(String, GameProcess)> {
        let games = self.running_games.lock().unwrap_or_else(|e| e.into_inner());

        games
            .iter()
            .filter_map(|(title, status)| {
                if let LaunchStatus::Running(process) = status {
                    Some((title.clone(), process.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Check for dead processes and update status
    pub fn check_processes(&self) -> Vec<String> {
        let games = self.running_games.lock().unwrap_or_else(|e| e.into_inner());
        let mut dead_games = Vec::new();

        for (title, status) in games.iter() {
            if let LaunchStatus::Running(process) = status {
                if !GameLauncher::is_process_running(process.main_pid) {
                    dead_games.push(title.clone());
                }
            }
        }

        // Drop lock before updating
        drop(games);

        // Remove dead games
        for title in &dead_games {
            self.on_process_exited(title);
        }

        dead_games
    }
}

impl Default for GameLaunchController {
    fn default() -> Self {
        Self::new()
    }
}

/// Messages from the launch controller
#[derive(Debug, Clone)]
pub enum LaunchMessage {
    /// Game launched successfully
    Launched(String, GameProcess),
    /// Game launch failed
    LaunchFailed(String, String),
    /// Process exited
    ProcessExited(String, u32),
    /// Launch progress update
    Progress(String, String),
}
