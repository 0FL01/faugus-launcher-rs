// Application configuration management
// Handles loading/saving of launcher settings

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;

use crate::config::paths::Paths;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Close launcher after game launch
    pub close_on_launch: bool,

    /// Default prefix directory
    pub default_prefix: PathBuf,

    /// Enable MangoHud by default
    pub mangohud: bool,

    /// Enable GameMode by default
    pub gamemode: bool,

    /// Disable hidraw support
    pub disable_hidraw: bool,

    /// Default Proton runner
    pub default_runner: String,

    /// Lossless scaling DLL location
    pub lossless_location: PathBuf,

    /// Use discrete GPU
    pub discrete_gpu: bool,

    /// Disable splash screen
    pub splash_disable: bool,

    /// Enable system tray
    pub system_tray: bool,

    /// Start at boot
    pub start_boot: bool,

    /// Use monochrome icon
    pub mono_icon: bool,

    /// Interface mode (List, Blocks, Banners)
    pub interface_mode: InterfaceMode,

    /// Start maximized
    pub start_maximized: bool,

    /// Start fullscreen
    pub start_fullscreen: bool,

    /// Show labels on banners
    pub show_labels: bool,

    /// Use smaller banners
    pub smaller_banners: bool,

    /// Enable logging
    pub enable_logging: bool,

    /// Use Wayland driver
    pub wayland_driver: bool,

    /// Enable HDR
    pub enable_hdr: bool,

    /// Enable WOW64
    pub enable_wow64: bool,

    /// Language code
    pub language: String,

    /// Show logging warning
    pub logging_warning: bool,

    /// Show hidden games
    pub show_hidden: bool,
}

/// Interface display modes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum InterfaceMode {
    #[default]
    List,
    Blocks,
    Banners,
}

impl InterfaceMode {
    pub const ALL: [InterfaceMode; 3] = [
        InterfaceMode::List,
        InterfaceMode::Blocks,
        InterfaceMode::Banners,
    ];
}

impl fmt::Display for InterfaceMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                InterfaceMode::List => "List",
                InterfaceMode::Blocks => "Grid",
                InterfaceMode::Banners => "Banner",
            }
        )
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            close_on_launch: false,
            default_prefix: Paths::default_prefix(),
            mangohud: false,
            gamemode: false,
            disable_hidraw: false,
            default_runner: String::from("GE-Proton"),
            lossless_location: PathBuf::new(),
            discrete_gpu: false,
            splash_disable: false,
            system_tray: false,
            start_boot: false,
            mono_icon: false,
            interface_mode: InterfaceMode::List,
            start_maximized: false,
            start_fullscreen: false,
            show_labels: false,
            smaller_banners: false,
            enable_logging: false,
            wayland_driver: false,
            enable_hdr: false,
            enable_wow64: false,
            language: String::from("en_US"),
            logging_warning: false,
            show_hidden: false,
        }
    }
}

impl AppConfig {
    /// Load configuration from config.ini file
    pub fn load() -> Result<Self> {
        let config_path = Paths::config_file();

        if !config_path.exists() {
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

        let mut config = Self::default();

        // Parse INI-like format
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim().trim_matches('"');

                match key {
                    "close-onlaunch" => config.close_on_launch = value.parse().unwrap_or(false),
                    "default-prefix" => config.default_prefix = PathBuf::from(value),
                    "mangohud" => config.mangohud = value.parse().unwrap_or(false),
                    "gamemode" => config.gamemode = value.parse().unwrap_or(false),
                    "disable-hidraw" => config.disable_hidraw = value.parse().unwrap_or(false),
                    "default-runner" => config.default_runner = value.to_string(),
                    "lossless-location" => config.lossless_location = PathBuf::from(value),
                    "discrete-gpu" => config.discrete_gpu = value.parse().unwrap_or(false),
                    "splash-disable" => config.splash_disable = value.parse().unwrap_or(false),
                    "system-tray" => config.system_tray = value.parse().unwrap_or(false),
                    "start-boot" => config.start_boot = value.parse().unwrap_or(false),
                    "mono-icon" => config.mono_icon = value.parse().unwrap_or(false),
                    "interface-mode" => {
                        config.interface_mode = match value {
                            "Blocks" => InterfaceMode::Blocks,
                            "Banners" => InterfaceMode::Banners,
                            _ => InterfaceMode::List,
                        };
                    }
                    "start-maximized" => config.start_maximized = value.parse().unwrap_or(false),
                    "start-fullscreen" => config.start_fullscreen = value.parse().unwrap_or(false),
                    "show-labels" => config.show_labels = value.parse().unwrap_or(false),
                    "smaller-banners" => config.smaller_banners = value.parse().unwrap_or(false),
                    "enable-logging" => config.enable_logging = value.parse().unwrap_or(false),
                    "wayland-driver" => config.wayland_driver = value.parse().unwrap_or(false),
                    "enable-hdr" => config.enable_hdr = value.parse().unwrap_or(false),
                    "enable-wow64" => config.enable_wow64 = value.parse().unwrap_or(false),
                    "language" => config.language = value.to_string(),
                    "logging-warning" => config.logging_warning = value.parse().unwrap_or(false),
                    "show-hidden" => config.show_hidden = value.parse().unwrap_or(false),
                    _ => {
                        tracing::warn!("Unknown config key: {}", key);
                    }
                }
            }
        }

        Ok(config)
    }

    /// Save configuration to config.ini file
    pub fn save(&self) -> Result<()> {
        let config_path = Paths::config_file();

        // Ensure parent directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }

        let mut content = String::from("# Faugus Launcher Configuration\n");

        // Write all configuration values
        content.push_str(&format!("close-onlaunch={}\n", self.close_on_launch));
        content.push_str(&format!(
            "default-prefix=\"{}\"\n",
            self.default_prefix.display()
        ));
        content.push_str(&format!("mangohud={}\n", self.mangohud));
        content.push_str(&format!("gamemode={}\n", self.gamemode));
        content.push_str(&format!("disable-hidraw={}\n", self.disable_hidraw));
        content.push_str(&format!("default-runner=\"{}\"\n", self.default_runner));
        content.push_str(&format!(
            "lossless-location=\"{}\"\n",
            self.lossless_location.display()
        ));
        content.push_str(&format!("discrete-gpu={}\n", self.discrete_gpu));
        content.push_str(&format!("splash-disable={}\n", self.splash_disable));
        content.push_str(&format!("system-tray={}\n", self.system_tray));
        content.push_str(&format!("start-boot={}\n", self.start_boot));
        content.push_str(&format!("mono-icon={}\n", self.mono_icon));
        content.push_str(&format!("interface-mode={:?}\n", self.interface_mode));
        content.push_str(&format!("start-maximized={}\n", self.start_maximized));
        content.push_str(&format!("start-fullscreen={}\n", self.start_fullscreen));
        content.push_str(&format!("show-labels={}\n", self.show_labels));
        content.push_str(&format!("smaller-banners={}\n", self.smaller_banners));
        content.push_str(&format!("enable-logging={}\n", self.enable_logging));
        content.push_str(&format!("wayland-driver={}\n", self.wayland_driver));
        content.push_str(&format!("enable-hdr={}\n", self.enable_hdr));
        content.push_str(&format!("enable-wow64={}\n", self.enable_wow64));
        content.push_str(&format!("language={}\n", self.language));
        content.push_str(&format!("logging-warning={}\n", self.logging_warning));
        content.push_str(&format!("show-hidden={}\n", self.show_hidden));

        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file: {:?}", config_path))?;

        Ok(())
    }

    /// Update configuration with new values
    /// TODO: Implement config update UI (settings dialog save)
    #[allow(dead_code)]
    pub fn update(&mut self, updates: ConfigUpdates) -> Result<()> {
        if let Some(close_on_launch) = updates.close_on_launch {
            self.close_on_launch = close_on_launch;
        }
        if let Some(default_prefix) = updates.default_prefix {
            self.default_prefix = default_prefix;
        }
        if let Some(mangohud) = updates.mangohud {
            self.mangohud = mangohud;
        }
        if let Some(gamemode) = updates.gamemode {
            self.gamemode = gamemode;
        }
        if let Some(disable_hidraw) = updates.disable_hidraw {
            self.disable_hidraw = disable_hidraw;
        }
        if let Some(default_runner) = updates.default_runner {
            self.default_runner = default_runner;
        }
        if let Some(lossless_location) = updates.lossless_location {
            self.lossless_location = lossless_location;
        }
        if let Some(discrete_gpu) = updates.discrete_gpu {
            self.discrete_gpu = discrete_gpu;
        }
        if let Some(splash_disable) = updates.splash_disable {
            self.splash_disable = splash_disable;
        }
        if let Some(system_tray) = updates.system_tray {
            self.system_tray = system_tray;
        }
        if let Some(start_boot) = updates.start_boot {
            self.start_boot = start_boot;
        }
        if let Some(mono_icon) = updates.mono_icon {
            self.mono_icon = mono_icon;
        }
        if let Some(interface_mode) = updates.interface_mode {
            self.interface_mode = interface_mode;
        }
        if let Some(start_maximized) = updates.start_maximized {
            self.start_maximized = start_maximized;
        }
        if let Some(start_fullscreen) = updates.start_fullscreen {
            self.start_fullscreen = start_fullscreen;
        }
        if let Some(show_labels) = updates.show_labels {
            self.show_labels = show_labels;
        }
        if let Some(smaller_banners) = updates.smaller_banners {
            self.smaller_banners = smaller_banners;
        }
        if let Some(enable_logging) = updates.enable_logging {
            self.enable_logging = enable_logging;
        }
        if let Some(wayland_driver) = updates.wayland_driver {
            self.wayland_driver = wayland_driver;
        }
        if let Some(enable_hdr) = updates.enable_hdr {
            self.enable_hdr = enable_hdr;
        }
        if let Some(enable_wow64) = updates.enable_wow64 {
            self.enable_wow64 = enable_wow64;
        }
        if let Some(language) = updates.language {
            self.language = language;
        }
        if let Some(logging_warning) = updates.logging_warning {
            self.logging_warning = logging_warning;
        }
        if let Some(show_hidden) = updates.show_hidden {
            self.show_hidden = show_hidden;
        }

        self.save()
    }
}

/// Partial configuration updates
/// TODO: Implement config update UI (settings dialog save)
#[allow(dead_code)]
pub struct ConfigUpdates {
    pub close_on_launch: Option<bool>,
    pub default_prefix: Option<PathBuf>,
    pub mangohud: Option<bool>,
    pub gamemode: Option<bool>,
    pub disable_hidraw: Option<bool>,
    pub default_runner: Option<String>,
    pub lossless_location: Option<PathBuf>,
    pub discrete_gpu: Option<bool>,
    pub splash_disable: Option<bool>,
    pub system_tray: Option<bool>,
    pub start_boot: Option<bool>,
    pub mono_icon: Option<bool>,
    pub interface_mode: Option<InterfaceMode>,
    pub start_maximized: Option<bool>,
    pub start_fullscreen: Option<bool>,
    pub show_labels: Option<bool>,
    pub smaller_banners: Option<bool>,
    pub enable_logging: Option<bool>,
    pub wayland_driver: Option<bool>,
    pub enable_hdr: Option<bool>,
    pub enable_wow64: Option<bool>,
    pub language: Option<String>,
    pub logging_warning: Option<bool>,
    pub show_hidden: Option<bool>,
}
