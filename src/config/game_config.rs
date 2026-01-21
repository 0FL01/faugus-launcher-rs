// Game configuration structure
// Represents a single game entry in the launcher

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::config::paths::Paths;

/// Format game title for use in filenames/IDs
/// Converts "Test's Game" -> "tests-game", "My Game" -> "my-game"
pub fn format_title(title: &str) -> String {
    title
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| match c {
            ' ' => '-',                    // spaces become hyphens
            c if c.is_alphanumeric() => c, // keep letters/numbers
            c if c == '-' => c,            // keep existing hyphens
            _ => '\u{0000}',               // mark other chars for removal (null char)
        })
        .filter(|&c| c != '\u{0000}') // remove marked chars
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

/// Legacy compatibility module for deserializing Python format games.json
mod legacy_compat {
    use serde::{Deserialize, Deserializer};
    use std::path::PathBuf;

    /// Deserialize bool from either:
    /// - Native bool (Rust format): true/false
    /// - String (Python format): "MANGOHUD=1", "gamemoderun", "addapp_enabled", etc.
    pub fn deserialize_bool_from_string_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum BoolOrString {
            Bool(bool),
            String(String),
        }

        match BoolOrString::deserialize(deserializer)? {
            BoolOrString::Bool(b) => Ok(b),
            BoolOrString::String(s) => Ok(!s.is_empty()),
        }
    }

    /// Deserialize u32 from either:
    /// - Native number (Rust format)
    /// - String (Python format): "2", "3", etc.
    pub fn deserialize_u32_from_string_or_number<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum NumberOrString {
            Number(u32),
            String(String),
        }

        match NumberOrString::deserialize(deserializer)? {
            NumberOrString::Number(n) => Ok(n),
            NumberOrString::String(s) => s
                .parse::<u32>()
                .map_err(|_| serde::de::Error::custom(format!("Invalid u32 string: {}", s))),
        }
    }

    /// Deserialize Option<PathBuf> from either:
    /// - null/missing (Rust format)
    /// - Empty string "" (Python format) -> None
    /// - Path string (both formats)
    pub fn deserialize_optional_path_from_string<'de, D>(
        deserializer: D,
    ) -> Result<Option<PathBuf>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum OptionalPath {
            Null,
            String(String),
        }

        Ok(match OptionalPath::deserialize(deserializer)? {
            OptionalPath::Null => None,
            OptionalPath::String(s) => {
                if s.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(s))
                }
            }
        })
    }
}

/// Represents a single game in the launcher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    /// Unique identifier for the game
    pub gameid: String,

    /// Display title
    pub title: String,

    /// Path to the game executable
    pub path: PathBuf,

    /// Wine prefix path
    pub prefix: PathBuf,

    /// Additional launch arguments
    pub launch_arguments: String,

    /// Game-specific arguments
    pub game_arguments: String,

    /// Enable MangoHud
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub mangohud: bool,

    /// Enable GameMode
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub gamemode: bool,

    /// Disable hidraw support
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub disable_hidraw: bool,

    /// Proton fix configuration
    pub protonfix: String,

    /// Proton runner to use
    pub runner: String,

    /// AddApp checkbox state
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub addapp_checkbox: bool,

    /// AddApp configuration
    pub addapp: String,

    /// AddApp batch file
    pub addapp_bat: String,

    /// Path to banner image
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_optional_path_from_string"
    )]
    pub banner: Option<PathBuf>,

    /// Lossless scaling enabled
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub lossless_enabled: bool,

    /// Lossless scaling multiplier
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_u32_from_string_or_number"
    )]
    pub lossless_multiplier: u32,

    /// Lossless scaling flow
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub lossless_flow: bool,

    /// Lossless scaling performance mode
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub lossless_performance: bool,

    /// Lossless scaling HDR
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub lossless_hdr: bool,

    /// Total playtime in seconds
    pub playtime: u64,

    /// Hidden from library
    pub hidden: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            gameid: uuid::Uuid::new_v4().to_string(),
            title: String::new(),
            path: PathBuf::new(),
            prefix: Paths::default_prefix(),
            launch_arguments: String::new(),
            game_arguments: String::new(),
            mangohud: false,
            gamemode: false,
            disable_hidraw: false,
            protonfix: String::new(),
            runner: String::from("GE-Proton"),
            addapp_checkbox: false,
            addapp: String::new(),
            addapp_bat: String::new(),
            banner: None,
            lossless_enabled: false,
            lossless_multiplier: 2,
            lossless_flow: false,
            lossless_performance: false,
            lossless_hdr: false,
            playtime: 0,
            hidden: false,
        }
    }
}

impl Game {
    /// Load all games from the games.json file
    pub fn load_all() -> Result<Vec<Self>> {
        let games_path = Paths::games_json();

        if !games_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&games_path)
            .with_context(|| format!("Failed to read games file: {:?}", games_path))?;

        let games: Vec<Self> =
            serde_json::from_str(&content).with_context(|| "Failed to parse games JSON")?;

        Ok(games)
    }

    /// Save all games to the games.json file
    pub fn save_all(games: &[Self]) -> Result<()> {
        let games_path = Paths::games_json();

        // Ensure parent directory exists
        if let Some(parent) = games_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create games directory: {:?}", parent))?;
        }

        let content =
            serde_json::to_string_pretty(games).with_context(|| "Failed to serialize games")?;

        fs::write(&games_path, content)
            .with_context(|| format!("Failed to write games file: {:?}", games_path))?;

        Ok(())
    }

    /// Save this game to the games list
    pub fn save(&self) -> Result<()> {
        let mut games = Self::load_all().unwrap_or_default();

        // Find and update existing game or add new one
        if let Some(existing) = games.iter().position(|g| g.gameid == self.gameid) {
            games[existing] = self.clone();
        } else {
            games.push(self.clone());
        }

        Self::save_all(&games)
    }

    /// Delete this game from the games list
    pub fn delete(&self) -> Result<()> {
        let mut games = Self::load_all().unwrap_or_default();

        games.retain(|g| g.gameid != self.gameid);

        Self::save_all(&games)
    }

    /// Format playtime as human-readable string
    pub fn format_playtime(&self) -> String {
        let hours = self.playtime / 3600;
        let minutes = (self.playtime % 3600) / 60;

        if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }

    /// Increment playtime by given seconds
    pub fn add_playtime(&mut self, seconds: u64) {
        self.playtime += seconds;
    }

    /// Update the hidden state of this game
    pub fn update_hidden(&self, hidden: bool) -> Result<()> {
        let mut games = Self::load_all().unwrap_or_default();

        // Find and update existing game
        if let Some(existing) = games.iter().position(|g| g.gameid == self.gameid) {
            games[existing].hidden = hidden;
            Self::save_all(&games)?;
        }

        Ok(())
    }

    /// Duplicate this game with a new ID and (Copy) suffix
    pub fn duplicate(&self) -> Self {
        Self {
            gameid: uuid::Uuid::new_v4().to_string(),
            title: format!("{} (Copy)", self.title),
            path: self.path.clone(),
            prefix: self.prefix.clone(),
            launch_arguments: self.launch_arguments.clone(),
            game_arguments: self.game_arguments.clone(),
            mangohud: self.mangohud,
            gamemode: self.gamemode,
            disable_hidraw: self.disable_hidraw,
            protonfix: self.protonfix.clone(),
            runner: self.runner.clone(),
            addapp_checkbox: self.addapp_checkbox,
            addapp: self.addapp.clone(),
            addapp_bat: self.addapp_bat.clone(),
            banner: self.banner.clone(),
            lossless_enabled: self.lossless_enabled,
            lossless_multiplier: self.lossless_multiplier,
            lossless_flow: self.lossless_flow,
            lossless_performance: self.lossless_performance,
            lossless_hdr: self.lossless_hdr,
            playtime: 0,
            hidden: false,
        }
    }
}

/// Game configuration for creation/editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub gameid: Option<String>,
    pub title: String,
    pub path: PathBuf,
    pub prefix: PathBuf,
    pub launch_arguments: String,
    pub game_arguments: String,
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub mangohud: bool,
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub gamemode: bool,
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub disable_hidraw: bool,
    pub protonfix: String,
    pub runner: String,
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub addapp_checkbox: bool,
    pub addapp: String,
    pub addapp_bat: String,
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_optional_path_from_string"
    )]
    pub banner: Option<PathBuf>,
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub lossless_enabled: bool,
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_u32_from_string_or_number"
    )]
    pub lossless_multiplier: u32,
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub lossless_flow: bool,
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub lossless_performance: bool,
    #[serde(
        default,
        deserialize_with = "legacy_compat::deserialize_bool_from_string_or_bool"
    )]
    pub lossless_hdr: bool,
}

impl From<GameConfig> for Game {
    fn from(config: GameConfig) -> Self {
        Self {
            gameid: config
                .gameid
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            title: config.title,
            path: config.path,
            prefix: config.prefix,
            launch_arguments: config.launch_arguments,
            game_arguments: config.game_arguments,
            mangohud: config.mangohud,
            gamemode: config.gamemode,
            disable_hidraw: config.disable_hidraw,
            protonfix: config.protonfix,
            runner: config.runner,
            addapp_checkbox: config.addapp_checkbox,
            addapp: config.addapp,
            addapp_bat: config.addapp_bat,
            banner: config.banner,
            lossless_enabled: config.lossless_enabled,
            lossless_multiplier: config.lossless_multiplier,
            lossless_flow: config.lossless_flow,
            lossless_performance: config.lossless_performance,
            lossless_hdr: config.lossless_hdr,
            playtime: 0,
            hidden: false,
        }
    }
}

impl From<Game> for GameConfig {
    fn from(game: Game) -> Self {
        Self {
            gameid: Some(game.gameid),
            title: game.title,
            path: game.path,
            prefix: game.prefix,
            launch_arguments: game.launch_arguments,
            game_arguments: game.game_arguments,
            mangohud: game.mangohud,
            gamemode: game.gamemode,
            disable_hidraw: game.disable_hidraw,
            protonfix: game.protonfix,
            runner: game.runner,
            addapp_checkbox: game.addapp_checkbox,
            addapp: game.addapp,
            addapp_bat: game.addapp_bat,
            banner: game.banner,
            lossless_enabled: game.lossless_enabled,
            lossless_multiplier: game.lossless_multiplier,
            lossless_flow: game.lossless_flow,
            lossless_performance: game.lossless_performance,
            lossless_hdr: game.lossless_hdr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_python_bool_formats() {
        let python_json = r#"[{
            "gameid": "test-game-1",
            "title": "Test Game",
            "path": "/path/to/game.exe",
            "prefix": "/home/user/Faugus",
            "launch_arguments": "",
            "game_arguments": "",
            "mangohud": "MANGOHUD=1",
            "gamemode": "gamemoderun",
            "disable_hidraw": "",
            "protonfix": "",
            "runner": "GE-Proton",
            "addapp_checkbox": "addapp_enabled",
            "addapp": "",
            "addapp_bat": "",
            "banner": "/path/to/banner.png",
            "lossless_enabled": "true",
            "lossless_multiplier": "2",
            "lossless_flow": "",
            "lossless_performance": "",
            "lossless_hdr": "",
            "playtime": 0,
            "hidden": false
        }]"#;

        let games: Vec<Game> =
            serde_json::from_str(python_json).expect("Failed to deserialize Python format JSON");

        assert_eq!(games.len(), 1);
        let game = &games[0];

        assert!(game.mangohud, "mangohud should be true from 'MANGOHUD=1'");
        assert!(game.gamemode, "gamemode should be true from 'gamemoderun'");
        assert!(
            !game.disable_hidraw,
            "disable_hidraw should be false from ''"
        );
        assert!(
            game.addapp_checkbox,
            "addapp_checkbox should be true from 'addapp_enabled'"
        );
        assert!(
            game.lossless_enabled,
            "lossless_enabled should be true from 'true'"
        );
        assert_eq!(
            game.lossless_multiplier, 2,
            "lossless_multiplier should be 2 from '2'"
        );
        assert!(!game.lossless_flow, "lossless_flow should be false from ''");
    }

    #[test]
    fn test_deserialize_rust_bool_formats() {
        let rust_json = r#"[{
            "gameid": "test-game-2",
            "title": "Test Game 2",
            "path": "/path/to/game2.exe",
            "prefix": "/home/user/Faugus",
            "launch_arguments": "",
            "game_arguments": "",
            "mangohud": true,
            "gamemode": false,
            "disable_hidraw": true,
            "protonfix": "",
            "runner": "GE-Proton",
            "addapp_checkbox": false,
            "addapp": "",
            "addapp_bat": "",
            "banner": null,
            "lossless_enabled": true,
            "lossless_multiplier": 3,
            "lossless_flow": true,
            "lossless_performance": false,
            "lossless_hdr": true,
            "playtime": 3600,
            "hidden": false
        }]"#;

        let games: Vec<Game> =
            serde_json::from_str(rust_json).expect("Failed to deserialize Rust format JSON");

        assert_eq!(games.len(), 1);
        let game = &games[0];

        assert!(game.mangohud, "mangohud should be true");
        assert!(!game.gamemode, "gamemode should be false");
        assert!(game.disable_hidraw, "disable_hidraw should be true");
        assert!(!game.addapp_checkbox, "addapp_checkbox should be false");
        assert!(game.lossless_enabled, "lossless_enabled should be true");
        assert_eq!(
            game.lossless_multiplier, 3,
            "lossless_multiplier should be 3"
        );
        assert!(game.lossless_flow, "lossless_flow should be true");
    }

    #[test]
    fn test_deserialize_banner_empty_string() {
        let json = r#"[{
            "gameid": "test-game-3",
            "title": "Test Game 3",
            "path": "/path/to/game3.exe",
            "prefix": "/home/user/Faugus",
            "launch_arguments": "",
            "game_arguments": "",
            "mangohud": false,
            "gamemode": false,
            "disable_hidraw": false,
            "protonfix": "",
            "runner": "GE-Proton",
            "addapp_checkbox": false,
            "addapp": "",
            "addapp_bat": "",
            "banner": "",
            "lossless_enabled": false,
            "lossless_multiplier": 2,
            "lossless_flow": false,
            "lossless_performance": false,
            "lossless_hdr": false,
            "playtime": 0,
            "hidden": false
        }]"#;

        let games: Vec<Game> =
            serde_json::from_str(json).expect("Failed to deserialize JSON with empty banner");

        assert_eq!(games.len(), 1);
        let game = &games[0];

        assert!(
            game.banner.is_none(),
            "banner should be None for empty string"
        );
    }

    #[test]
    fn test_serialize_to_rust_format() {
        let game = Game {
            gameid: "test-game-4".to_string(),
            title: "Test Game 4".to_string(),
            path: PathBuf::from("/path/to/game4.exe"),
            prefix: PathBuf::from("/home/user/Faugus"),
            launch_arguments: String::new(),
            game_arguments: String::new(),
            mangohud: true,
            gamemode: true,
            disable_hidraw: false,
            protonfix: String::new(),
            runner: "GE-Proton".to_string(),
            addapp_checkbox: true,
            addapp: String::new(),
            addapp_bat: String::new(),
            banner: Some(PathBuf::from("/path/to/banner.png")),
            lossless_enabled: true,
            lossless_multiplier: 2,
            lossless_flow: false,
            lossless_performance: false,
            lossless_hdr: false,
            playtime: 0,
            hidden: false,
        };

        let json = serde_json::to_string(&game).expect("Failed to serialize game to JSON");
        let parsed: serde_json::Value =
            serde_json::from_str(&json).expect("Failed to parse serialized JSON");

        assert!(
            parsed["mangohud"].is_boolean(),
            "mangohud should serialize as bool"
        );
        assert_eq!(parsed["mangohud"], true);
        assert!(
            parsed["gamemode"].is_boolean(),
            "gamemode should serialize as bool"
        );
        assert_eq!(parsed["gamemode"], true);
        assert!(
            parsed["lossless_multiplier"].is_number(),
            "lossless_multiplier should serialize as number"
        );
        assert_eq!(parsed["lossless_multiplier"], 2);
    }
}
