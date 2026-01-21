// Configuration module
// Manages application configuration, game data, and paths

pub mod app_config;
pub mod game_config;
pub mod paths;

pub use app_config::{AppConfig, InterfaceMode};
pub use game_config::{format_title, Game};
