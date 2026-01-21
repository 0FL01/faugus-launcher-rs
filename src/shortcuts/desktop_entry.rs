// Desktop Entry management
// Handles .desktop file creation and management for XDG Desktop Entry

use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use tracing::{info, warn};

use crate::config::game_config::{format_title, Game};
use crate::config::paths::Paths;
use crate::shortcuts::ShortcutLocation;

/// Desktop entry file format
#[derive(Debug, Clone)]
pub struct DesktopEntry {
    /// Name of the application
    pub name: String,
    /// Executable command
    pub exec: String,
    /// Icon path
    pub icon: String,
    /// Working directory
    pub path: String,
    /// Categories
    pub categories: Vec<String>,
    /// Comment/description
    pub comment: Option<String>,
    /// Terminal (true if app runs in terminal)
    pub terminal: bool,
}

impl DesktopEntry {
    /// Create a new desktop entry for a game
    pub fn for_game(game: &Game) -> Result<Self> {
        // Get faugus-run binary path
        let faugus_run = Paths::faugus_run().context("faugus-run binary not found in PATH")?;

        // Build launch command
        let exec = Self::build_launch_command(game, &faugus_run);

        // Get icon path
        let icon = Self::get_icon_path(game);

        // Get working directory
        let path = if let Some(parent) = game.path.parent() {
            parent.to_string_lossy().to_string()
        } else {
            ".".to_string()
        };

        Ok(Self {
            name: game.title.clone(),
            exec,
            icon,
            path,
            categories: vec!["Game".to_string()],
            comment: Some(format!("Launch {} with Faugus Launcher", game.title)),
            terminal: false,
        })
    }

    /// Build the launch command for the game
    fn build_launch_command(game: &Game, faugus_run: &std::path::Path) -> String {
        let mut parts = Vec::new();

        // Add environment variables
        if game.mangohud {
            parts.push("MANGOHUD=1".to_string());
        }
        if game.disable_hidraw {
            parts.push("PROTON_DISABLE_HIDRAW=1".to_string());
        }

        // Add GAMEID
        let game_id = if !game.protonfix.is_empty() {
            game.protonfix.clone()
        } else {
            format_title(&game.title)
        };
        parts.push(format!("GAMEID={}", game_id));

        // Add GameMode
        if game.gamemode {
            parts.push("gamemoderun".to_string());
        }

        // Add launch arguments
        if !game.launch_arguments.is_empty() {
            parts.push(game.launch_arguments.clone());
        }

        // Add Lossless Scaling settings
        if game.lossless_enabled {
            parts.push("LSFG_LEGACY=1".to_string());
            if game.lossless_multiplier > 0 {
                parts.push(format!("LSFG_MULTIPLIER={}", game.lossless_multiplier));
            }
            if game.lossless_flow {
                parts.push(format!("LSFG_FLOW_SCALE={}", game.lossless_flow));
            }
            if game.lossless_performance {
                parts.push("LSFG_PERFORMANCE_MODE=1".to_string());
            }
            if game.lossless_hdr {
                parts.push("LSFG_HDR_MODE=1".to_string());
            }
        }

        // Add umu-run command
        parts.push(format!("'{}'", faugus_run.to_string_lossy()));

        // Add game path
        let escaped_path = game.path.to_string_lossy().replace("'", "'\\''");
        parts.push(format!("'{}'", escaped_path));

        // Add game arguments
        if !game.game_arguments.is_empty() {
            parts.push(game.game_arguments.clone());
        }

        // Join all parts
        parts.join(" ")
    }

    /// Get icon path for the game
    fn get_icon_path(game: &Game) -> String {
        // Check for game icon in icons directory
        let icon_path = Paths::icons_dir().join(format!("{}.png", game.gameid));
        if icon_path.exists() {
            return icon_path.to_string_lossy().to_string();
        }

        // Check for game icon with formatted title
        let formatted_title = format_title(&game.title);
        let icon_path = Paths::icons_dir().join(format!("{}.png", formatted_title));
        if icon_path.exists() {
            return icon_path.to_string_lossy().to_string();
        }

        // Check for .ico version
        let ico_path = Paths::icons_dir().join(format!("{}.ico", formatted_title));
        if ico_path.exists() {
            return ico_path.to_string_lossy().to_string();
        }

        // Fall back to Faugus Launcher icon
        let faugus_icon = Paths::get_icon("faugus-launcher.png");
        if faugus_icon.exists() {
            return faugus_icon.to_string_lossy().to_string();
        }

        // Final fallback
        "faugus-launcher".to_string()
    }

    /// Convert to .desktop file content
    pub fn to_desktop_file(&self) -> String {
        let mut content = String::from("[Desktop Entry]\n");
        content.push_str(&format!("Name={}\n", self.name));
        content.push_str(&format!("Exec={}\n", self.exec));
        content.push_str(&format!("Icon={}\n", self.icon));
        content.push_str("Type=Application\n");
        content.push_str(&format!("Path={}\n", self.path));

        if !self.categories.is_empty() {
            content.push_str(&format!("Categories={};\n", self.categories.join(";")));
        }

        if let Some(ref comment) = self.comment {
            content.push_str(&format!("Comment={}\n", comment));
        }

        if self.terminal {
            content.push_str("Terminal=true\n");
        } else {
            content.push_str("Terminal=false\n");
        }

        content
    }

    /// Get the filename for this desktop entry
    pub fn filename(&self) -> String {
        format!("{}.desktop", format_title(&self.name))
    }
}

/// Desktop shortcut manager
pub struct DesktopShortcutManager;

impl DesktopShortcutManager {
    /// Create shortcuts for a game
    pub fn create(game: &Game, location: ShortcutLocation) -> Result<()> {
        info!("Creating desktop shortcuts for: {}", game.title);

        let entry = DesktopEntry::for_game(game)?;
        let content = entry.to_desktop_file();
        let filename = entry.filename();

        // Create applications menu shortcut
        if matches!(
            location,
            ShortcutLocation::Applications | ShortcutLocation::Both
        ) {
            Self::create_applications_shortcut(&filename, &content)?;
        }

        // Create desktop shortcut
        if matches!(location, ShortcutLocation::Desktop | ShortcutLocation::Both) {
            Self::create_desktop_shortcut(&filename, &content)?;
        }

        Ok(())
    }

    /// Create application menu shortcut
    fn create_applications_shortcut(filename: &str, content: &str) -> Result<()> {
        let applications_dir = Paths::applications_dir();

        // Ensure directory exists
        fs::create_dir_all(&applications_dir).context("Failed to create applications directory")?;

        let shortcut_path = applications_dir.join(filename);

        // Write desktop file
        fs::write(&shortcut_path, content).context("Failed to write applications shortcut")?;

        // Make executable
        let mut perms = fs::metadata(&shortcut_path)
            .context("Failed to get shortcut permissions")?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&shortcut_path, perms).context("Failed to set shortcut permissions")?;

        info!("Created applications shortcut: {:?}", shortcut_path);
        Ok(())
    }

    /// Create desktop shortcut
    fn create_desktop_shortcut(filename: &str, content: &str) -> Result<()> {
        let desktop_dir = Paths::desktop_dir();

        // Ensure directory exists
        fs::create_dir_all(&desktop_dir).context("Failed to create desktop directory")?;

        let shortcut_path = desktop_dir.join(filename);

        // Write desktop file
        fs::write(&shortcut_path, content).context("Failed to write desktop shortcut")?;

        // Make executable
        let mut perms = fs::metadata(&shortcut_path)
            .context("Failed to get shortcut permissions")?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&shortcut_path, perms).context("Failed to set shortcut permissions")?;

        info!("Created desktop shortcut: {:?}", shortcut_path);
        Ok(())
    }

    /// Remove shortcuts for a game
    pub fn remove(game: &Game) -> Result<()> {
        info!("Removing desktop shortcuts for: {}", game.title);

        let formatted_title = format_title(&game.title);
        let filename = format!("{}.desktop", formatted_title);

        let applications_dir = Paths::applications_dir();
        let desktop_dir = Paths::desktop_dir();

        let mut removed = false;

        // Remove from applications menu
        let app_shortcut = applications_dir.join(&filename);
        if app_shortcut.exists() {
            fs::remove_file(&app_shortcut).context("Failed to remove applications shortcut")?;
            info!("Removed applications shortcut: {:?}", app_shortcut);
            removed = true;
        }

        // Remove from desktop
        let desktop_shortcut = desktop_dir.join(&filename);
        if desktop_shortcut.exists() {
            fs::remove_file(&desktop_shortcut).context("Failed to remove desktop shortcut")?;
            info!("Removed desktop shortcut: {:?}", desktop_shortcut);
            removed = true;
        }

        if !removed {
            warn!("No shortcuts found for game: {}", game.title);
        }

        Ok(())
    }

    /// Check if shortcuts exist for a game
    pub fn exists(game: &Game) -> bool {
        let formatted_title = format_title(&game.title);
        let filename = format!("{}.desktop", formatted_title);

        let app_shortcut = Paths::applications_dir().join(&filename);
        let desktop_shortcut = Paths::desktop_dir().join(&filename);

        app_shortcut.exists() || desktop_shortcut.exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_title() {
        assert_eq!(format_title("My Game"), "my-game");
        assert_eq!(format_title("Test's Game"), "tests-game");
        assert_eq!(format_title("  Spaces  "), "spaces");
    }
}
