// System Tray Integration
// Provides system tray icon and menu functionality for Faugus Launcher

mod icon;
mod menu;
mod tray;

pub use tray::{SystemTray, TrayEvent};

use std::path::PathBuf;

/// Configuration for system tray
#[derive(Debug, Clone)]
pub struct TrayConfig {
    /// Enable system tray icon
    pub enabled: bool,
    /// Start minimized to tray
    pub start_minimized: bool,
    /// Close to tray instead of quitting
    pub close_to_tray: bool,
    /// Show notifications
    pub show_notifications: bool,
    /// Use monochrome icon
    pub is_mono: bool,
    /// Custom icon path (optional)
    pub icon_path: Option<PathBuf>,
}

impl Default for TrayConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            start_minimized: false,
            close_to_tray: false,
            show_notifications: true,
            is_mono: false,
            icon_path: None,
        }
    }
}
