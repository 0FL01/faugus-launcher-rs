// System Tray Implementation
// Main system tray functionality using tray-icon crate

use anyhow::{Context, Result};
use std::sync::mpsc;
use tracing::info;

use super::icon::TrayIcon;
use super::menu::TrayMenu;
use super::TrayConfig;

/// Events from the system tray
#[derive(Debug, Clone)]
pub enum TrayEvent {
    /// Show window requested
    Show,
    /// Hide window requested
    Hide,
    /// Quit application requested
    Quit,
    /// Tray icon clicked
    TrayIconClicked,
    /// Tray icon double-clicked
    TrayIconDoubleClicked,
}

/// Messages for tray communication
#[derive(Debug, Clone)]
pub enum TrayMessage {
    /// Set window visibility state
    SetWindowVisible(bool),
    /// Show notification
    ShowNotification { title: String, body: String },
    /// Set tooltip text
    SetTooltip(String),
    /// Quit tray
    Quit,
}

/// System tray manager
pub struct SystemTray {
    config: TrayConfig,
    icon: Option<TrayIcon>,
    menu: Option<TrayMenu>,
    tray_icon: Option<tray_icon::TrayIcon>,
    event_tx: Option<mpsc::Sender<TrayEvent>>,
    window_visible: bool,
}

impl SystemTray {
    /// Create a new system tray instance
    pub fn new(config: TrayConfig) -> Self {
        Self {
            config,
            icon: None,
            menu: None,
            tray_icon: None,
            event_tx: None,
            window_visible: true,
        }
    }

    /// Initialize the system tray
    pub fn init(&mut self, i18n: &crate::locale::I18n) -> Result<()> {
        if !self.config.enabled {
            info!("System tray disabled in config");
            return Ok(());
        }

        // Load tray icon
        self.icon = Some(TrayIcon::new(
            self.config.icon_path.clone(),
            self.config.is_mono,
        )?);

        // Create menu
        self.menu = Some(TrayMenu::new(i18n)?);

        // Create tray icon
        let tray_icon = self.create_tray_icon()?;
        self.tray_icon = Some(tray_icon);

        info!("System tray initialized");
        Ok(())
    }

    /// Create the actual tray icon
    fn create_tray_icon(&self) -> Result<tray_icon::TrayIcon> {
        let icon = self.icon.as_ref().context("Tray icon not loaded")?;
        let menu = self.menu.as_ref().context("Tray menu not created")?;

        // Load tray_icon::Icon
        let tray_icon_data = icon.load_icon()?;

        // Create tray icon using builder
        let tray_icon = tray_icon::TrayIconBuilder::new()
            .with_menu(Box::new(menu.menu().clone()))
            .with_tooltip("Faugus Launcher")
            .with_icon(tray_icon_data)
            .build()
            .context("Failed to create tray icon")?;

        Ok(tray_icon)
    }

    /// Set the event sender for tray events
    pub fn set_event_sender(&mut self, tx: mpsc::Sender<TrayEvent>) {
        self.event_tx = Some(tx);
    }

    /// Handle tray messages
    pub fn handle_message(&mut self, message: TrayMessage) -> Result<()> {
        match message {
            TrayMessage::SetWindowVisible(visible) => {
                self.window_visible = visible;
                if let Some(menu) = &self.menu {
                    menu.set_window_visible(visible);
                }
                let _ = self.update_tooltip();
            }
            TrayMessage::ShowNotification { title, body } => {
                if self.config.show_notifications {
                    let _ = self.show_notification(&title, &body);
                }
            }
            TrayMessage::SetTooltip(tooltip) => {
                if let Some(tray_icon) = &self.tray_icon {
                    let _ = tray_icon.set_tooltip(Some(tooltip));
                }
            }
            TrayMessage::Quit => {
                self.cleanup();
            }
        }
        Ok(())
    }

    /// Update the tray icon tooltip based on window state
    fn update_tooltip(&self) -> Option<String> {
        if let Some(tray_icon) = &self.tray_icon {
            let tooltip = if self.window_visible {
                "Faugus Launcher - Visible".to_string()
            } else {
                "Faugus Launcher - Hidden".to_string()
            };
            let _ = tray_icon.set_tooltip(Some(&tooltip));
            Some(tooltip)
        } else {
            None
        }
    }

    /// Show a system notification
    fn show_notification(&self, title: &str, body: &str) -> Result<()> {
        notify_rust::Notification::new()
            .summary(title)
            .body(body)
            .show()?;
        Ok(())
    }

    /// Check if tray is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled && self.tray_icon.is_some()
    }

    /// Clean up tray resources
    pub fn cleanup(&mut self) {
        if let Some(tray_icon) = self.tray_icon.take() {
            drop(tray_icon);
        }
        info!("System tray cleaned up");
    }

    /// Set window visibility state
    pub fn set_window_visible(&mut self, visible: bool) {
        self.window_visible = visible;
        if let Some(menu) = &self.menu {
            menu.set_window_visible(visible);
        }
        let _ = self.update_tooltip();
    }
}

impl Drop for SystemTray {
    fn drop(&mut self) {
        self.cleanup();
    }
}

/// Default implementation
impl Default for SystemTray {
    fn default() -> Self {
        Self::new(TrayConfig::default())
    }
}
