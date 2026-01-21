// Steam shortcuts management
// Handles reading, writing, and modifying Steam shortcuts.vdf

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

use crate::config::paths::Paths;
use crate::config::Game;

/// Steam shortcut entry
/// TODO: Use for Steam shortcut import/export feature
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SteamShortcut {
    pub appid: u32,
    pub appname: String,
    pub exe: String,
    pub start_dir: String,
    pub icon: String,
    pub shortcut_path: String,
    pub launch_options: String,
    pub is_hidden: u32,
    pub allow_desktop_config: u32,
    pub allow_overlay: u32,
    pub open_vr: u32,
    pub devkit: u32,
    pub devkit_game_id: String,
    pub devkit_override_appid: u32,
    pub last_play_time: u64,
    pub auto_close_shortcut: u32,
    pub dip_treat_remote_desktop_as_sitting_in_front_of_monitor: u32,
    pub fandom_tags: Vec<String>,
}

impl Default for SteamShortcut {
    fn default() -> Self {
        Self {
            appid: 0,
            appname: String::new(),
            exe: String::new(),
            start_dir: String::new(),
            icon: String::new(),
            shortcut_path: String::new(),
            launch_options: String::new(),
            is_hidden: 0,
            allow_desktop_config: 1,
            allow_overlay: 1,
            open_vr: 0,
            devkit: 0,
            devkit_game_id: String::new(),
            devkit_override_appid: 0,
            last_play_time: 0,
            auto_close_shortcut: 0,
            dip_treat_remote_desktop_as_sitting_in_front_of_monitor: 0,
            fandom_tags: Vec::new(),
        }
    }
}

/// Steam shortcuts manager
pub struct SteamShortcuts {
    shortcuts: Map<String, Value>,
    vdf_path: PathBuf,
}

impl SteamShortcuts {
    /// Load shortcuts from the Steam shortcuts.vdf file
    pub fn load() -> Result<Self> {
        let vdf_path = Paths::steam_shortcuts_vdf()
            .context("Steam shortcuts.vdf not found. Is Steam installed?")?;

        info!("Loading Steam shortcuts from: {:?}", vdf_path);

        if !vdf_path.exists() {
            warn!("shortcuts.vdf does not exist, creating new shortcuts map");
            return Ok(Self {
                shortcuts: Map::new(),
                vdf_path,
            });
        }

        // Use new-vdf-parser to read binary VDF
        let shortcuts_value = new_vdf_parser::open_shortcuts_vdf(&vdf_path);

        let shortcuts: Map<String, Value> = if let Some(obj) = shortcuts_value.as_object() {
            obj.clone()
        } else {
            Map::new()
        };

        info!("Loaded {} Steam shortcuts", shortcuts.len());

        Ok(Self {
            shortcuts,
            vdf_path,
        })
    }

    /// Save shortcuts to the Steam shortcuts.vdf file
    pub fn save(&self) -> Result<()> {
        info!(
            "Saving {} Steam shortcuts to: {:?}",
            self.shortcuts.len(),
            self.vdf_path
        );

        // Ensure parent directory exists
        if let Some(parent) = self.vdf_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create shortcuts.vdf parent directory")?;
        }

        // Convert to Value
        let value = Value::Object(self.shortcuts.clone());

        // Use new-vdf-parser to write binary VDF
        let success = new_vdf_parser::write_shortcuts_vdf(&self.vdf_path, value);

        if !success {
            anyhow::bail!("Failed to write shortcuts.vdf");
        }

        info!("Successfully saved Steam shortcuts");
        Ok(())
    }

    /// Find an existing shortcut by app name
    fn find_shortcut(&self, appname: &str) -> Option<(String, Map<String, Value>)> {
        for (key, value) in &self.shortcuts {
            if let Some(obj) = value.as_object() {
                if let Some(name) = obj.get("AppName").and_then(|v| v.as_str()) {
                    if name == appname {
                        return Some((key.clone(), obj.clone()));
                    }
                }
            }
        }
        None
    }

    /// Generate a new unique app ID
    fn generate_appid(&self) -> u32 {
        let mut max_id = 0u32;

        for key in self.shortcuts.keys() {
            if let Ok(id) = key.parse::<u32>() {
                if id > max_id {
                    max_id = id;
                }
            }
        }

        // Start from a high number to avoid conflicts
        max_id.max(1000000) + 1
    }

    /// Add or update a game in Steam shortcuts
    pub fn add_or_update(&mut self, game: &Game) -> Result<bool> {
        info!("Adding/updating Steam shortcut for: {}", game.title);

        // Get faugus-run path
        let faugus_run = Paths::faugus_run().context("faugus-run binary not found in PATH")?;

        // Build icon path
        let icon = Paths::icons_dir().join(format!("{}.png", game.gameid));
        let icon_str = if icon.exists() {
            icon.to_string_lossy().to_string()
        } else {
            String::new()
        };

        // Build launch options
        let launch_options = format!("--game {}", game.gameid);

        // Build exe path (quoted)
        let exe = format!("\"{}\"", faugus_run.to_string_lossy());

        // Build start dir
        let start_dir = if let Some(parent) = game.path.parent() {
            parent.to_string_lossy().to_string()
        } else {
            ".".to_string()
        };

        // Check if shortcut already exists
        if let Some((existing_key, _existing_obj)) = self.find_shortcut(&game.title) {
            info!("Updating existing shortcut: {}", game.title);

            // Update existing shortcut
            if let Some(obj) = self.shortcuts.get_mut(&existing_key) {
                if let Some(obj_map) = obj.as_object_mut() {
                    obj_map.insert("Exe".to_string(), Value::String(exe.clone()));
                    obj_map.insert("LaunchOptions".to_string(), Value::String(launch_options));
                    obj_map.insert("StartDir".to_string(), Value::String(start_dir.clone()));
                    obj_map.insert("icon".to_string(), Value::String(icon_str.clone()));
                    obj_map.insert("AppName".to_string(), Value::String(game.title.clone()));
                }
            }

            Ok(true)
        } else {
            info!("Creating new shortcut: {}", game.title);

            // Create new shortcut
            let appid = self.generate_appid();

            let mut shortcut = Map::new();
            shortcut.insert(
                "appid".to_string(),
                Value::Number(serde_json::Number::from(appid)),
            );
            shortcut.insert("AppName".to_string(), Value::String(game.title.clone()));
            shortcut.insert("Exe".to_string(), Value::String(exe));
            shortcut.insert("StartDir".to_string(), Value::String(start_dir));
            shortcut.insert("icon".to_string(), Value::String(icon_str));
            shortcut.insert("ShortcutPath".to_string(), Value::String(String::new()));
            shortcut.insert("LaunchOptions".to_string(), Value::String(launch_options));
            shortcut.insert(
                "IsHidden".to_string(),
                Value::Number(serde_json::Number::from(0u32)),
            );
            shortcut.insert(
                "AllowDesktopConfig".to_string(),
                Value::Number(serde_json::Number::from(1u32)),
            );
            shortcut.insert(
                "AllowOverlay".to_string(),
                Value::Number(serde_json::Number::from(1u32)),
            );
            shortcut.insert(
                "OpenVR".to_string(),
                Value::Number(serde_json::Number::from(0u32)),
            );
            shortcut.insert(
                "Devkit".to_string(),
                Value::Number(serde_json::Number::from(0u32)),
            );
            shortcut.insert("DevkitGameID".to_string(), Value::String(String::new()));
            shortcut.insert(
                "DevkitOverrideAppID".to_string(),
                Value::Number(serde_json::Number::from(0u32)),
            );
            shortcut.insert(
                "LastPlayTime".to_string(),
                Value::Number(serde_json::Number::from(0u64)),
            );
            shortcut.insert(
                "AutoCloseShortcut".to_string(),
                Value::Number(serde_json::Number::from(0u32)),
            );
            shortcut.insert("FlatpakAppID".to_string(), Value::String(String::new()));

            self.shortcuts
                .insert(appid.to_string(), Value::Object(shortcut));

            Ok(false)
        }
    }

    /// Remove a game from Steam shortcuts
    pub fn remove(&mut self, title: &str) -> Result<bool> {
        info!("Removing Steam shortcut for: {}", title);

        if let Some((key, _)) = self.find_shortcut(title) {
            self.shortcuts.remove(&key);
            info!("Removed shortcut: {}", title);
            Ok(true)
        } else {
            warn!("Shortcut not found: {}", title);
            Ok(false)
        }
    }

    /// Check if a game is in Steam shortcuts
    pub fn contains(&self, title: &str) -> bool {
        self.find_shortcut(title).is_some()
    }

    /// Get all shortcuts
    /// TODO: Use for Steam shortcut UI (import/export)
    #[allow(dead_code)]
    pub fn get_all(&self) -> Vec<SteamShortcut> {
        let mut shortcuts = Vec::new();

        for (_key, value) in &self.shortcuts {
            if let Some(obj) = value.as_object() {
                if let Ok(shortcut) = self.value_to_shortcut(obj) {
                    shortcuts.push(shortcut);
                }
            }
        }

        shortcuts.sort_by(|a, b| a.appname.cmp(&b.appname));
        shortcuts
    }

    /// Convert Value to SteamShortcut
    /// TODO: Use for Steam shortcut UI (import/export)
    #[allow(dead_code)]
    fn value_to_shortcut(&self, obj: &Map<String, Value>) -> Result<SteamShortcut> {
        Ok(SteamShortcut {
            appid: obj.get("appid").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            appname: obj
                .get("AppName")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            exe: obj
                .get("Exe")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            start_dir: obj
                .get("StartDir")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            icon: obj
                .get("icon")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            shortcut_path: obj
                .get("ShortcutPath")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            launch_options: obj
                .get("LaunchOptions")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            is_hidden: obj.get("IsHidden").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            allow_desktop_config: obj
                .get("AllowDesktopConfig")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32,
            allow_overlay: obj
                .get("AllowOverlay")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32,
            open_vr: obj.get("OpenVR").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            devkit: obj.get("Devkit").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
            devkit_game_id: obj
                .get("DevkitGameID")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            devkit_override_appid: obj
                .get("DevkitOverrideAppID")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32,
            last_play_time: obj
                .get("LastPlayTime")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            auto_close_shortcut: obj
                .get("AutoCloseShortcut")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32,
            dip_treat_remote_desktop_as_sitting_in_front_of_monitor: obj
                .get("DipTreatRemoteDesktopAsSittingInFrontOfMonitor")
                .and_then(|v| v.as_u64())
                .unwrap_or(0)
                as u32,
            fandom_tags: obj
                .get("tags")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steam_shortcut_default() {
        let shortcut = SteamShortcut::default();
        assert_eq!(shortcut.appid, 0);
        assert_eq!(shortcut.appname, "");
        assert_eq!(shortcut.is_hidden, 0);
        assert_eq!(shortcut.allow_desktop_config, 1);
    }
}
