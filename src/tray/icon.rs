// Tray Icon Handler
// Manages the system tray icon

use anyhow::{Context, Result};
use std::path::PathBuf;
use tracing::{info, warn};

use crate::config::paths::Paths;

/// Tray icon manager
pub struct TrayIcon {
    icon_path: PathBuf,
}

impl TrayIcon {
    /// Create a new tray icon manager
    pub fn new(custom_path: Option<PathBuf>, is_mono: bool) -> Result<Self> {
        let icon_path = if let Some(path) = custom_path {
            if path.exists() {
                path
            } else {
                warn!("Custom tray icon not found: {:?}, using default", path);
                Self::find_default_icon(is_mono)?
            }
        } else {
            Self::find_default_icon(is_mono)?
        };

        info!("Using tray icon: {:?}", icon_path);

        Ok(Self { icon_path })
    }

    /// Get the icon path
    /// TODO: Use for icon refresh, custom icon support
    #[allow(dead_code)]
    pub fn path(&self) -> &PathBuf {
        &self.icon_path
    }

    /// Find the default Faugus Launcher icon
    fn find_default_icon(is_mono: bool) -> Result<PathBuf> {
        if let Some(icon) = Paths::get_app_icon(is_mono) {
            return Ok(icon);
        }

        Err(anyhow::anyhow!("No suitable tray icon found"))
    }

    /// Load icon as tray_icon::Icon
    pub fn load_icon(&self) -> Result<tray_icon::Icon> {
        let img = image::open(&self.icon_path).context("Failed to open tray icon image")?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        let rgba_raw = rgba.into_raw();

        tray_icon::Icon::from_rgba(rgba_raw, width, height)
            .context("Failed to create tray icon from RGBA")
    }
}
