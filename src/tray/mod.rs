// System Tray Integration
// Provides system tray icon and menu functionality for Faugus Launcher

// tray.rs module contains SystemTray implementation
#![allow(clippy::module_inception)]

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
    /// TODO: Implement startup behavior (minimized to tray)
    #[allow(dead_code)]
    pub start_minimized: bool,
    /// TODO: Implement close-to-tray behavior
    #[allow(dead_code)]
    pub close_to_tray: bool,
    /// TODO: Implement tray notifications
    #[allow(dead_code)]
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
