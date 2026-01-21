// Path management
// Handles XDG paths and application directories

use std::path::PathBuf;
use std::{env, fs};

/// Path management utilities
pub struct Paths;

impl Paths {
    /// Get XDG DATA HOME directory
    fn xdg_data_home() -> PathBuf {
        env::var("XDG_DATA_HOME")
            .ok()
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
                PathBuf::from(home).join(".local/share")
            })
    }

    /// Get XDG CONFIG HOME directory
    fn xdg_config_home() -> PathBuf {
        env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
                PathBuf::from(home).join(".config")
            })
    }

    /// Get XDG DATA DIRS (system data directories)
    fn xdg_data_dirs() -> Vec<PathBuf> {
        env::var("XDG_DATA_DIRS")
            .ok()
            .unwrap_or_else(|| "/usr/local/share:/usr/share".to_string())
            .split(':')
            .map(PathBuf::from)
            .collect()
    }

    /// Get system data directory
    pub fn system_data(relative_path: &str) -> Option<PathBuf> {
        let _parts: Vec<&str> = relative_path.split('/').collect();

        for data_dir in Self::xdg_data_dirs() {
            let path = data_dir.join(relative_path);
            if path.exists() {
                return Some(path);
            }
        }

        // Fallback to first data dir
        Self::xdg_data_dirs()
            .first()
            .map(|dir| dir.join(relative_path))
    }

    /// Get user data directory
    pub fn user_data(relative_path: &str) -> PathBuf {
        Self::xdg_data_home().join(relative_path)
    }

    /// Get user config directory
    pub fn user_config(relative_path: &str) -> PathBuf {
        Self::xdg_config_home().join(relative_path)
    }

    /// Find binary in PATH
    pub fn find_binary(name: &str) -> Option<PathBuf> {
        if let Ok(path_var) = env::var("PATH") {
            for path in path_var.split(':') {
                let binary = PathBuf::from(path).join(name);
                if binary.exists() {
                    return Some(binary);
                }
            }
        }
        None
    }

    /// Get icon path
    pub fn get_icon(icon_name: &str) -> PathBuf {
        // Check user icons first
        let user_icons = Self::user_data(&format!("icons/{}", icon_name));
        if user_icons.exists() {
            return user_icons;
        }

        // Check system icon themes
        let system_icons = Self::system_data(&format!("icons/hicolor/256x256/apps/{}", icon_name));
        if let Some(path) = system_icons {
            if path.exists() {
                return path;
            }
        }

        // Check system icons
        let system_icons = Self::system_data(&format!("icons/{}", icon_name));
        if let Some(path) = system_icons {
            if path.exists() {
                return path;
            }
        }

        // Check pixmaps
        let pixmap_path = PathBuf::from("/usr/share/pixmaps").join(icon_name);
        if pixmap_path.exists() {
            return pixmap_path;
        }

        // Check assets directory (for development)
        let asset_paths = vec![
            PathBuf::from("assets").join(icon_name),
            PathBuf::from("../assets").join(icon_name),
        ];

        for path in asset_paths {
            if path.exists() {
                return path;
            }
        }

        // Fallback
        PathBuf::from(icon_name)
    }

    /// Get application icon path (for window icon)
    pub fn get_app_icon(is_mono: bool) -> Option<PathBuf> {
        let icon_name = if is_mono {
            "faugus-mono.png"
        } else {
            "faugus-launcher.png"
        };

        let path = Self::get_icon(icon_name);
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    /// Application-specific paths
    pub fn config_dir() -> PathBuf {
        Self::user_config("faugus-launcher")
    }

    pub fn config_file() -> PathBuf {
        Self::config_dir().join("config.ini")
    }

    pub fn games_json() -> PathBuf {
        Self::config_dir().join("games.json")
    }

    pub fn latest_games_txt() -> PathBuf {
        Self::config_dir().join("latest-games.txt")
    }

    pub fn icons_dir() -> PathBuf {
        Self::config_dir().join("icons")
    }

    pub fn banners_dir() -> PathBuf {
        Self::config_dir().join("banners")
    }

    pub fn logs_dir() -> PathBuf {
        Self::config_dir().join("logs")
    }

    /// Get path to envar.txt (legacy compatibility)
    pub fn envar_txt() -> PathBuf {
        Self::config_dir().join("envar.txt")
    }

    /// Get default WinePREFIX path
    pub fn default_prefix() -> PathBuf {
        let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
        PathBuf::from(home).join("Faugus")
    }

    pub fn running_games_json() -> PathBuf {
        Self::user_data("faugus-launcher/running_games.json")
    }

    /// Steam paths
    pub fn steam_userdata_paths() -> Vec<PathBuf> {
        let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
        let home_path = PathBuf::from(&home);

        vec![
            home_path.join(".local/share/Steam/userdata"),
            home_path.join(".steam/steam/userdata"),
            home_path.join(".steam/root/userdata"),
            PathBuf::from(format!(
                "{}/.var/app/com.valvesoftware.Steam/.steam/steam/userdata/",
                home
            )),
        ]
    }

    pub fn steam_userdata_path() -> Option<PathBuf> {
        Self::steam_userdata_paths()
            .into_iter()
            .find(|path| path.exists())
    }

    pub fn steam_id() -> Option<String> {
        if let Some(userdata_path) = Self::steam_userdata_path() {
            if let Ok(entries) = fs::read_dir(userdata_path) {
                for entry in entries.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        if name.chars().all(|c| c.is_ascii_digit()) {
                            return Some(name);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn steam_shortcuts_vdf() -> Option<PathBuf> {
        if let Some(steam_id) = Self::steam_id() {
            if let Some(userdata_path) = Self::steam_userdata_path() {
                return Some(userdata_path.join(&steam_id).join("config/shortcuts.vdf"));
            }
        }
        None
    }

    /// Steam compatibility tools directory
    pub fn steam_compat_tools_dir() -> PathBuf {
        Self::xdg_data_home()
            .join("Steam")
            .join("compatibilitytools.d")
    }

    /// Desktop directory
    pub fn desktop_dir() -> PathBuf {
        // Try xdg-user-dir first
        if let Ok(output) = std::process::Command::new("xdg-user-dir")
            .arg("DESKTOP")
            .output()
        {
            if output.status.success() {
                if let Ok(path_str) = String::from_utf8(output.stdout) {
                    let path = path_str.trim().to_string();
                    if !path.is_empty() {
                        return PathBuf::from(path);
                    }
                }
            }
        }

        // Fallback to ~/Desktop
        let home = env::var("HOME").unwrap_or_else(|_| "/".to_string());
        PathBuf::from(home).join("Desktop")
    }

    /// Applications directory
    pub fn applications_dir() -> PathBuf {
        Self::xdg_data_home().join("applications")
    }

    /// Get faugus-run binary path
    pub fn faugus_run() -> Option<PathBuf> {
        Self::find_binary("faugus-run")
    }

    /// Get umu-run path
    pub fn umu_run() -> PathBuf {
        Self::user_data("faugus-launcher/umu-run")
    }

    /// Get mangohud binary
    pub fn mangohud() -> Option<PathBuf> {
        Self::find_binary("mangohud")
    }

    /// Get gamemoderun binary
    pub fn gamemoderun() -> Option<PathBuf> {
        Self::find_binary("gamemoderun")
    }
}
