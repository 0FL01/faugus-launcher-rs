// Icon manager
// Handles icon extraction from executables and image management

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tracing::{info, warn};

use crate::config::paths::Paths;

/// Icon sizes for different use cases
/// TODO: Implement icon size selection in UI (grid view, list view)
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum IconSize {
    Small = 32,
    Medium = 64,
    Large = 128,
    XLarge = 256,
}

/// Icon manager for extracting and managing game icons
pub struct IconManager;

impl IconManager {
    /// Extract icon from a Windows executable
    pub fn extract_from_exe(exe_path: &std::path::Path, game_id: &str) -> Result<PathBuf> {
        info!("Extracting icon from: {:?}", exe_path);

        // Create icons directory if it doesn't exist
        let icons_dir = Paths::icons_dir();
        fs::create_dir_all(&icons_dir).context("Failed to create icons directory")?;

        // Output icon path
        let icon_path = icons_dir.join(format!("{}.png", game_id));

        // Check if icon already exists
        if icon_path.exists() {
            info!("Icon already exists: {:?}", icon_path);
            return Ok(icon_path);
        }

        // Try to extract icon using wrestool (from icoutils package)
        if let Ok(path) = Self::extract_with_wrestool(exe_path, game_id) {
            return Ok(path);
        }

        // Fallback: try icoutils wrestool with different approach
        if let Ok(path) = Self::extract_with_icoutils(exe_path, game_id) {
            return Ok(path);
        }

        // Last resort: use default icon
        warn!("Failed to extract icon from {:?}, using default", exe_path);
        Self::copy_default_icon(game_id)
    }

    /// Extract icon using wrestool (preferred method)
    fn extract_with_wrestool(exe_path: &std::path::Path, game_id: &str) -> Result<PathBuf> {
        let icons_dir = Paths::icons_dir();
        let temp_ico = icons_dir.join(format!("{}_temp.ico", game_id));
        let icon_path = icons_dir.join(format!("{}.png", game_id));

        // Extract icon group to .ico file
        let output = Command::new("wrestool")
            .arg("-x")
            .arg("-t")
            .arg("14")
            .arg("-n")
            .arg("1") // Get first icon
            .arg("-o")
            .arg(&temp_ico)
            .arg(exe_path)
            .output();

        match output {
            Ok(result) if result.status.success() => {
                // Convert .ico to .png using icotool or ImageMagick
                if temp_ico.exists() && Self::convert_ico_to_png(&temp_ico, &icon_path).is_ok() {
                    // Clean up temp file
                    let _ = fs::remove_file(&temp_ico);
                    return Ok(icon_path);
                }
            }
            _ => {
                warn!("wrestool failed to extract icon");
            }
        }

        anyhow::bail!("wrestool extraction failed")
    }

    /// Extract icon using icoutils alternative method
    fn extract_with_icoutils(exe_path: &std::path::Path, game_id: &str) -> Result<PathBuf> {
        let icons_dir = Paths::icons_dir();
        let temp_dir = icons_dir.join(format!("{}_temp", game_id));

        // Create temp directory
        fs::create_dir_all(&temp_dir)?;

        // Extract all icons
        let output = Command::new("wrestool")
            .arg("-x")
            .arg("-t")
            .arg("14")
            .arg("-o")
            .arg(&temp_dir)
            .arg(exe_path)
            .output();

        if output.ok().is_some_and(|r| r.status.success()) {
            // Find the largest .ico file
            if let Some(largest_ico) = Self::find_largest_icon(&temp_dir) {
                let icon_path = icons_dir.join(format!("{}.png", game_id));
                if Self::convert_ico_to_png(&largest_ico, &icon_path).is_ok() {
                    // Clean up temp directory
                    let _ = fs::remove_dir_all(&temp_dir);
                    return Ok(icon_path);
                }
            }
        }

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
        anyhow::bail!("icoutils extraction failed")
    }

    /// Find the largest icon file in a directory
    fn find_largest_icon(dir: &std::path::Path) -> Option<PathBuf> {
        let entries = fs::read_dir(dir).ok()?;
        let mut largest = None;
        let mut largest_size = 0;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "ico") {
                if let Ok(metadata) = fs::metadata(&path) {
                    let size = metadata.len();
                    if size > largest_size {
                        largest_size = size;
                        largest = Some(path);
                    }
                }
            }
        }

        largest
    }

    /// Convert .ico file to .png
    fn convert_ico_to_png(ico_path: &std::path::Path, png_path: &std::path::Path) -> Result<()> {
        let png_dir = png_path
            .parent()
            .context("PNG path has no parent directory")?;

        // Try using icotool first (part of icoutils)
        if let Ok(result) = Command::new("icotool")
            .arg("-x")
            .arg("-o")
            .arg(png_dir)
            .arg(ico_path)
            .output()
        {
            if result.status.success() {
                // icotool creates multiple PNG files, find the largest
                if let Some(largest) = Self::find_largest_png(png_dir) {
                    if largest != *png_path {
                        fs::copy(&largest, png_path)?;
                        // Clean up extracted files
                        let _ = fs::remove_file(&largest);
                    }
                    return Ok(());
                }
            }
        }

        // Fallback: try ImageMagick convert/magick
        let magick_commands = vec!["magick", "convert"];
        for cmd in magick_commands {
            if let Ok(result) = Command::new(cmd)
                .arg(ico_path)
                .arg("-strip")
                .arg(png_path)
                .output()
            {
                if result.status.success() {
                    return Ok(());
                }
            }
        }

        // Last resort: just copy the ico file and rename it
        if ico_path.exists() {
            fs::copy(ico_path, png_path)?;
            return Ok(());
        }

        anyhow::bail!("Failed to convert icon to PNG")
    }

    /// Find the largest PNG file in a directory
    fn find_largest_png(dir: &std::path::Path) -> Option<PathBuf> {
        let entries = fs::read_dir(dir).ok()?;
        let mut largest = None;
        let mut largest_size = 0;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "png") {
                if let Ok(metadata) = fs::metadata(&path) {
                    let size = metadata.len();
                    if size > largest_size {
                        largest_size = size;
                        largest = Some(path);
                    }
                }
            }
        }

        largest
    }

    /// Copy a custom icon for a game
    pub fn copy_custom_icon(source: &std::path::Path, game_id: &str) -> Result<PathBuf> {
        let icons_dir = Paths::icons_dir();
        fs::create_dir_all(&icons_dir)?;

        let icon_path = icons_dir.join(format!("{}.png", game_id));

        // If source is not PNG, try to convert it
        if source.extension().is_some_and(|e| e == "ico") {
            Self::convert_ico_to_png(source, &icon_path)?;
        } else {
            fs::copy(source, &icon_path)?;
        }

        Ok(icon_path)
    }

    /// Get icon path for a game
    pub fn get_icon_path(game_id: &str) -> PathBuf {
        Paths::icons_dir().join(format!("{}.png", game_id))
    }

    /// Check if icon exists for a game
    pub fn icon_exists(game_id: &str) -> bool {
        Self::get_icon_path(game_id).exists()
    }

    /// Copy default icon for a game
    fn copy_default_icon(game_id: &str) -> Result<PathBuf> {
        let icons_dir = Paths::icons_dir();
        fs::create_dir_all(&icons_dir)?;

        let icon_path = icons_dir.join(format!("{}.png", game_id));

        // Try to find Faugus Launcher icon in assets directory
        let asset_paths = vec![
            PathBuf::from("assets/faugus-launcher.png"),
            PathBuf::from("../assets/faugus-launcher.png"),
            Paths::get_icon("faugus-launcher.png"),
        ];

        for asset_path in asset_paths {
            if asset_path.exists() {
                fs::copy(&asset_path, &icon_path)?;
                info!("Using default icon from: {:?}", asset_path);
                return Ok(icon_path);
            }
        }

        // If no default icon found, create a placeholder
        // This won't happen in production as assets are included
        warn!("No default icon found, creating placeholder");
        Ok(icon_path)
    }

    /// Delete icon for a game
    pub fn delete_icon(game_id: &str) -> Result<()> {
        let icon_path = Self::get_icon_path(game_id);
        if icon_path.exists() {
            fs::remove_file(&icon_path).context("Failed to remove icon")?;
            info!("Deleted icon: {:?}", icon_path);
        }
        Ok(())
    }

    /// Load icon as image handle for Iced
    /// TODO: Use for UI banner images, game cover art
    #[allow(dead_code)]
    pub fn load_icon(game_id: &str) -> Option<iced::widget::image::Handle> {
        let icon_path = Self::get_icon_path(game_id);
        if icon_path.exists() {
            Some(iced::widget::image::Handle::from_path(icon_path))
        } else {
            None
        }
    }

    /// Get or extract icon for a game
    /// TODO: Use for batch icon extraction on import
    #[allow(dead_code)]
    pub fn get_or_extract_icon(exe_path: &std::path::Path, game_id: &str) -> PathBuf {
        if !Self::icon_exists(game_id) {
            if let Err(e) = Self::extract_from_exe(exe_path, game_id) {
                warn!("Failed to extract icon: {}", e);
                let _ = Self::copy_default_icon(game_id);
            }
        }
        Self::get_icon_path(game_id)
    }

    /// Update icon for a game (re-extract from exe or use custom)
    /// TODO: Use for icon refresh in edit dialog
    #[allow(dead_code)]
    pub fn update_icon(exe_path: &std::path::Path, game_id: &str) -> Result<PathBuf> {
        // Remove old icon
        let _ = Self::delete_icon(game_id);

        // Extract new icon
        Self::extract_from_exe(exe_path, game_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_size_values() {
        assert_eq!(IconSize::Small as u32, 32);
        assert_eq!(IconSize::Medium as u32, 64);
        assert_eq!(IconSize::Large as u32, 128);
        assert_eq!(IconSize::XLarge as u32, 256);
    }

    #[test]
    fn test_icon_path_generation() {
        let path = IconManager::get_icon_path("test-game-id");
        assert!(path.to_string_lossy().ends_with("test-game-id.png"));
        assert!(path.to_string_lossy().contains("icons"));
    }
}
