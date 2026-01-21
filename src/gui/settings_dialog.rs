// Settings Dialog
// Configuration dialog for Faugus Launcher

use iced::widget::{
    button, checkbox, column, container, pick_list, row, scrollable, text, text_input, Space,
};
use iced::{Element, Length, Padding, Task};
use std::fmt;
use std::path::PathBuf;

use crate::config::{AppConfig, InterfaceMode};
use crate::gui::file_picker;
use crate::gui::styles::DeepSpace;
use crate::locale::I18n;
use crate::proton::proton_manager::ProtonManager;

/// Messages for the Settings dialog
#[derive(Debug, Clone)]
pub enum SettingsMessage {
    // General Settings
    LanguageChanged(Language),
    InterfaceModeChanged(InterfaceMode),
    StartMaximizedToggled(bool),
    StartFullscreenToggled(bool),
    ShowLabelsToggled(bool),
    SmallerBannersToggled(bool),
    MonoIconToggled(bool),
    ShowHiddenToggled(bool),

    // Path Settings
    DefaultPrefixChanged(String),
    BrowseDefaultPrefix,
    DefaultPrefixPicked(Option<PathBuf>),
    LosslessLocationChanged(String),
    BrowseLosslessLocation,
    LosslessLocationPicked(Option<PathBuf>),
    DefaultRunnerChanged(String),

    // Performance Settings
    MangoHudToggled(bool),
    GameModeToggled(bool),
    DisableHidrawToggled(bool),
    DiscreteGpuToggled(bool),

    // System Settings
    SystemTrayToggled(bool),
    StartBootToggled(bool),
    CloseOnLaunchToggled(bool),
    SplashDisableToggled(bool),
    EnableLoggingToggled(bool),

    // Experimental Settings
    WaylandDriverToggled(bool),
    EnableHdrToggled(bool),
    EnableWow64Toggled(bool),

    // Actions
    ProtonManagerClicked,
    WinetricksClicked,
    WinecfgClicked,
    RunClicked,
    BackupClicked,
    RestoreClicked,
    ClearLogsClicked,
    ShowLogsClicked,
    ResetToDefaults,

    // Dialog
    Confirm,
    Cancel,
}

/// Language option for the pick list
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language {
    pub code: String,
    pub name: String,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// State for the Settings dialog
#[derive(Debug, Clone)]
pub struct SettingsDialog {
    /// Current working configuration
    config: AppConfig,

    // Working values for editing
    language_index: usize,
    interface_mode_index: usize,
    runner_index: usize,

    // Available options
    languages: Vec<Language>,
    interface_modes: Vec<InterfaceMode>,
    runners: Vec<String>,

    // Dialog state
    logging_warning_shown: bool,

    /// Needs restart flag
    needs_restart: bool,
}

impl SettingsDialog {
    /// Create a new Settings dialog
    pub fn new(config: AppConfig) -> Self {
        let languages = Self::get_supported_languages();
        let language_index = languages
            .iter()
            .position(|l| l.code == config.language)
            .unwrap_or(0);

        let interface_modes = vec![
            InterfaceMode::List,
            InterfaceMode::Blocks,
            InterfaceMode::Banners,
        ];
        let interface_mode_index = interface_modes
            .iter()
            .position(|mode| mode == &config.interface_mode)
            .unwrap_or(0);

        let runners = ProtonManager::new().get_available_runners();
        let runner_index = runners
            .iter()
            .position(|r| r == &config.default_runner)
            .unwrap_or(0);

        Self {
            config,
            language_index,
            interface_mode_index,
            runner_index,
            languages,
            interface_modes,
            runners,
            logging_warning_shown: false,
            needs_restart: false,
        }
    }

    /// Get supported languages
    fn get_supported_languages() -> Vec<Language> {
        vec![
            Language {
                code: "af".to_string(),
                name: "Afrikaans".to_string(),
            },
            Language {
                code: "ar".to_string(),
                name: "Arabic".to_string(),
            },
            Language {
                code: "de".to_string(),
                name: "German".to_string(),
            },
            Language {
                code: "el".to_string(),
                name: "Greek".to_string(),
            },
            Language {
                code: "en_US".to_string(),
                name: "English".to_string(),
            },
            Language {
                code: "es".to_string(),
                name: "Spanish".to_string(),
            },
            Language {
                code: "fa".to_string(),
                name: "Persian".to_string(),
            },
            Language {
                code: "fr".to_string(),
                name: "French".to_string(),
            },
            Language {
                code: "hu".to_string(),
                name: "Hungarian".to_string(),
            },
            Language {
                code: "it".to_string(),
                name: "Italian".to_string(),
            },
            Language {
                code: "nl".to_string(),
                name: "Dutch".to_string(),
            },
            Language {
                code: "pl".to_string(),
                name: "Polish".to_string(),
            },
            Language {
                code: "pt_BR".to_string(),
                name: "Portuguese (Brazil)".to_string(),
            },
            Language {
                code: "ru".to_string(),
                name: "Russian".to_string(),
            },
            Language {
                code: "sv".to_string(),
                name: "Swedish".to_string(),
            },
            Language {
                code: "uk".to_string(),
                name: "Ukrainian".to_string(),
            },
            Language {
                code: "zh_CN".to_string(),
                name: "Chinese (Simplified)".to_string(),
            },
        ]
    }

    /// Update the dialog state
    pub fn update(&mut self, message: SettingsMessage) -> Task<SettingsMessage> {
        match message {
            SettingsMessage::LanguageChanged(lang) => {
                if let Some(idx) = self.languages.iter().position(|l| l.code == lang.code) {
                    self.language_index = idx;
                    self.config.language = lang.code;
                }
            }
            SettingsMessage::InterfaceModeChanged(mode) => {
                if let Some(idx) = self.interface_modes.iter().position(|m| m == &mode) {
                    self.interface_mode_index = idx;
                    self.config.interface_mode = mode;
                    self.needs_restart = true;
                }
            }
            SettingsMessage::StartMaximizedToggled(enabled) => {
                self.config.start_maximized = enabled;
            }
            SettingsMessage::StartFullscreenToggled(enabled) => {
                self.config.start_fullscreen = enabled;
            }
            SettingsMessage::ShowLabelsToggled(enabled) => {
                self.config.show_labels = enabled;
                self.needs_restart = true;
            }
            SettingsMessage::SmallerBannersToggled(enabled) => {
                self.config.smaller_banners = enabled;
                self.needs_restart = true;
            }
            SettingsMessage::MonoIconToggled(enabled) => {
                self.config.mono_icon = enabled;
                self.needs_restart = true;
            }
            SettingsMessage::ShowHiddenToggled(enabled) => {
                self.config.show_hidden = enabled;
            }
            SettingsMessage::DefaultPrefixChanged(path) => {
                self.config.default_prefix = PathBuf::from(path);
            }
            SettingsMessage::BrowseDefaultPrefix => {
                return Task::perform(
                    file_picker::pick_folder(),
                    SettingsMessage::DefaultPrefixPicked,
                );
            }
            SettingsMessage::DefaultPrefixPicked(path) => {
                if let Some(path) = path {
                    self.config.default_prefix = path;
                }
            }
            SettingsMessage::LosslessLocationChanged(path) => {
                self.config.lossless_location = PathBuf::from(path);
            }
            SettingsMessage::BrowseLosslessLocation => {
                return Task::perform(
                    file_picker::pick_file(),
                    SettingsMessage::LosslessLocationPicked,
                );
            }
            SettingsMessage::LosslessLocationPicked(path) => {
                if let Some(path) = path {
                    self.config.lossless_location = path;
                }
            }
            SettingsMessage::DefaultRunnerChanged(runner) => {
                if let Some(idx) = self.runners.iter().position(|r| r == &runner) {
                    self.runner_index = idx;
                    self.config.default_runner = runner;
                }
            }
            SettingsMessage::MangoHudToggled(enabled) => {
                self.config.mangohud = enabled;
            }
            SettingsMessage::GameModeToggled(enabled) => {
                self.config.gamemode = enabled;
            }
            SettingsMessage::DisableHidrawToggled(enabled) => {
                self.config.disable_hidraw = enabled;
            }
            SettingsMessage::DiscreteGpuToggled(enabled) => {
                self.config.discrete_gpu = enabled;
            }
            SettingsMessage::SystemTrayToggled(enabled) => {
                self.config.system_tray = enabled;
                self.needs_restart = true;
            }
            SettingsMessage::StartBootToggled(enabled) => {
                self.config.start_boot = enabled;
            }
            SettingsMessage::CloseOnLaunchToggled(enabled) => {
                self.config.close_on_launch = enabled;
            }
            SettingsMessage::SplashDisableToggled(enabled) => {
                self.config.splash_disable = enabled;
            }
            SettingsMessage::EnableLoggingToggled(enabled) => {
                if enabled && !self.logging_warning_shown {
                    self.logging_warning_shown = true;
                    // Would show warning here
                }
                self.config.enable_logging = enabled;
            }
            SettingsMessage::WaylandDriverToggled(enabled) => {
                self.config.wayland_driver = enabled;
            }
            SettingsMessage::EnableHdrToggled(enabled) => {
                self.config.enable_hdr = enabled;
            }
            SettingsMessage::EnableWow64Toggled(enabled) => {
                self.config.enable_wow64 = enabled;
            }
            SettingsMessage::ProtonManagerClicked => {
                tracing::info!("Open Proton Manager");
                // Signal to open Proton Manager dialog
                // This would be handled by the parent through a callback
            }
            SettingsMessage::WinetricksClicked => {
                tracing::info!("Run Winetricks");
                // TODO: Execute winetricks
            }
            SettingsMessage::WinecfgClicked => {
                tracing::info!("Run Winecfg");
                // TODO: Execute winecfg
            }
            SettingsMessage::RunClicked => {
                tracing::info!("Run in prefix");
                // TODO: Open run dialog
            }
            SettingsMessage::BackupClicked => {
                tracing::info!("Backup settings");
                // TODO: Backup configuration
            }
            SettingsMessage::RestoreClicked => {
                tracing::info!("Restore settings");
                // TODO: Restore configuration
            }
            SettingsMessage::ClearLogsClicked => {
                tracing::info!("Clear logs");
                // TODO: Clear log files
            }
            SettingsMessage::ShowLogsClicked => {
                tracing::info!("Show logs");
                // Signal to open Log Viewer dialog
                // This would be handled by the parent through a callback
            }
            SettingsMessage::ResetToDefaults => {
                self.config = AppConfig::default();
                self.language_index = self
                    .languages
                    .iter()
                    .position(|l| l.code == self.config.language)
                    .unwrap_or(0);
                self.interface_mode_index = self
                    .interface_modes
                    .iter()
                    .position(|m| m == &self.config.interface_mode)
                    .unwrap_or(0);
                self.runner_index = self
                    .runners
                    .iter()
                    .position(|r| r == &self.config.default_runner)
                    .unwrap_or(0);
            }
            SettingsMessage::Confirm | SettingsMessage::Cancel => {
                // Handled by the caller
            }
        }
        Task::none()
    }

    /// Get the updated configuration
    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    /// Check if restart is needed
    pub fn needs_restart(&self) -> bool {
        self.needs_restart
    }

    /// View the dialog
    pub fn view(&self, i18n: &I18n) -> Element<'_, SettingsMessage> {
        let general_section = self.view_general_section(i18n);
        let paths_section = self.view_paths_section(i18n);
        let performance_section = self.view_performance_section(i18n);
        let system_section = self.view_system_section(i18n);
        let experimental_section = self.view_experimental_section(i18n);
        let tools_section = self.view_tools_section(i18n);
        let actions_section = self.view_actions_section(i18n);
        let buttons_section = self.view_buttons(i18n);

        let restart_notice = if self.needs_restart {
            column![
                Space::with_height(Length::Fixed(10.0)),
                text(i18n.t("Some changes require a restart to take effect."))
                    .size(12)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color::new(1.0, 0.6, 0.0, 1.0)),
                    }),
            ]
        } else {
            column![]
        };

        let scrollable = scrollable(
            column![
                general_section,
                Space::with_height(Length::Fixed(20.0)),
                paths_section,
                Space::with_height(Length::Fixed(20.0)),
                performance_section,
                Space::with_height(Length::Fixed(20.0)),
                system_section,
                Space::with_height(Length::Fixed(20.0)),
                experimental_section,
                Space::with_height(Length::Fixed(20.0)),
                tools_section,
                Space::with_height(Length::Fixed(20.0)),
                actions_section,
                restart_notice,
            ]
            .padding(Padding {
                top: 5.0,
                right: 15.0,
                bottom: 5.0,
                left: 15.0,
            }),
        )
        .width(Length::Fill)
        .height(Length::FillPortion(1))
        .style(DeepSpace::scrollable);

        let content = column![
            scrollable,
            Space::with_height(Length::Fixed(10.0)),
            buttons_section
        ]
        .spacing(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(DeepSpace::modal_container)
            .into()
    }

    /// View general settings section
    fn view_general_section(&self, i18n: &I18n) -> Element<'_, SettingsMessage> {
        column![
            text(i18n.t("General")).size(18),
            Space::with_height(Length::Fixed(10.0)),
            // Language
            column![
                text(i18n.t("Language")).size(14),
                Space::with_height(Length::Fixed(5.0)),
                pick_list(
                    &self.languages[..],
                    self.languages.get(self.language_index).cloned(),
                    SettingsMessage::LanguageChanged
                )
                .width(Length::Fill)
                .style(DeepSpace::pick_list)
                .menu_style(DeepSpace::menu),
            ]
            .spacing(5),
            Space::with_height(Length::Fixed(10.0)),
            // Interface Mode
            column![
                text(i18n.t("Interface Mode")).size(14),
                Space::with_height(Length::Fixed(5.0)),
                pick_list(
                    &InterfaceMode::ALL[..],
                    Some(self.config.interface_mode),
                    SettingsMessage::InterfaceModeChanged
                )
                .width(Length::Fill)
                .style(DeepSpace::pick_list)
                .menu_style(DeepSpace::menu),
            ]
            .spacing(5),
            Space::with_height(Length::Fixed(10.0)),
            // Checkboxes
            checkbox(i18n.t("Start maximized"), self.config.start_maximized)
                .on_toggle(SettingsMessage::StartMaximizedToggled)
                .style(DeepSpace::checkbox),
            checkbox(i18n.t("Start in fullscreen"), self.config.start_fullscreen)
                .on_toggle(SettingsMessage::StartFullscreenToggled)
                .style(DeepSpace::checkbox),
            checkbox(i18n.t("Show labels"), self.config.show_labels)
                .on_toggle(SettingsMessage::ShowLabelsToggled)
                .style(DeepSpace::checkbox),
            checkbox(i18n.t("Smaller banners"), self.config.smaller_banners)
                .on_toggle(SettingsMessage::SmallerBannersToggled)
                .style(DeepSpace::checkbox),
            checkbox(i18n.t("Monochrome icon"), self.config.mono_icon)
                .on_toggle(SettingsMessage::MonoIconToggled)
                .style(DeepSpace::checkbox),
            checkbox(i18n.t("Show hidden games"), self.config.show_hidden)
                .on_toggle(SettingsMessage::ShowHiddenToggled)
                .style(DeepSpace::checkbox),
        ]
        .spacing(5)
        .into()
    }

    /// View paths settings section
    fn view_paths_section(&self, i18n: &I18n) -> Element<'_, SettingsMessage> {
        let prefix_display = self.config.default_prefix.display().to_string();
        let lossless_display = self.config.lossless_location.display().to_string();

        column![
            text(i18n.t("Paths")).size(18),
            Space::with_height(Length::Fixed(10.0)),
            // Default Prefix
            column![
                text(i18n.t("Default Prefix Location")).size(14),
                Space::with_height(Length::Fixed(5.0)),
                row![
                    text_input("", &prefix_display)
                        .on_input(SettingsMessage::DefaultPrefixChanged)
                        .style(DeepSpace::text_input),
                    button(text("..."))
                        .on_press(SettingsMessage::BrowseDefaultPrefix)
                        .width(Length::Fixed(50.0))
                        .style(DeepSpace::button),
                ]
                .spacing(5),
            ]
            .spacing(5),
            Space::with_height(Length::Fixed(10.0)),
            // Lossless Scaling
            column![
                text(i18n.t("Lossless Scaling Location")).size(14),
                Space::with_height(Length::Fixed(5.0)),
                row![
                    text_input("", &lossless_display)
                        .on_input(SettingsMessage::LosslessLocationChanged)
                        .style(DeepSpace::text_input),
                    button(text("..."))
                        .on_press(SettingsMessage::BrowseLosslessLocation)
                        .width(Length::Fixed(50.0))
                        .style(DeepSpace::button),
                ]
                .spacing(5),
            ]
            .spacing(5),
            Space::with_height(Length::Fixed(10.0)),
            // Default Runner
            column![
                text(i18n.t("Default Proton")).size(14),
                Space::with_height(Length::Fixed(5.0)),
                pick_list(
                    &self.runners[..],
                    self.runners.get(self.runner_index).cloned(),
                    SettingsMessage::DefaultRunnerChanged
                )
                .width(Length::Fill)
                .style(DeepSpace::pick_list)
                .menu_style(DeepSpace::menu),
            ]
            .spacing(5),
        ]
        .spacing(5)
        .into()
    }

    /// View performance settings section
    fn view_performance_section(&self, i18n: &I18n) -> Element<'_, SettingsMessage> {
        column![
            text(i18n.t("Performance")).size(18),
            Space::with_height(Length::Fixed(10.0)),
            checkbox("MangoHud", self.config.mangohud)
                .on_toggle(SettingsMessage::MangoHudToggled)
                .style(DeepSpace::checkbox),
            checkbox("GameMode", self.config.gamemode)
                .on_toggle(SettingsMessage::GameModeToggled)
                .style(DeepSpace::checkbox),
            checkbox(i18n.t("Disable Hidraw"), self.config.disable_hidraw)
                .on_toggle(SettingsMessage::DisableHidrawToggled)
                .style(DeepSpace::checkbox),
            checkbox(i18n.t("Use discrete GPU"), self.config.discrete_gpu)
                .on_toggle(SettingsMessage::DiscreteGpuToggled)
                .style(DeepSpace::checkbox),
        ]
        .spacing(5)
        .into()
    }

    /// View system settings section
    fn view_system_section(&self, i18n: &I18n) -> Element<'_, SettingsMessage> {
        column![
            text(i18n.t("System")).size(18),
            Space::with_height(Length::Fixed(10.0)),
            checkbox(i18n.t("System tray icon"), self.config.system_tray)
                .on_toggle(SettingsMessage::SystemTrayToggled)
                .style(DeepSpace::checkbox),
            checkbox(i18n.t("Start on boot"), self.config.start_boot)
                .on_toggle(SettingsMessage::StartBootToggled)
                .style(DeepSpace::checkbox),
            checkbox(
                i18n.t("Close when running a game/app"),
                self.config.close_on_launch
            )
            .on_toggle(SettingsMessage::CloseOnLaunchToggled)
            .style(DeepSpace::checkbox),
            checkbox(i18n.t("Disable splash window"), self.config.splash_disable)
                .on_toggle(SettingsMessage::SplashDisableToggled)
                .style(DeepSpace::checkbox),
            checkbox(i18n.t("Enable logging"), self.config.enable_logging)
                .on_toggle(SettingsMessage::EnableLoggingToggled)
                .style(DeepSpace::checkbox),
        ]
        .spacing(5)
        .into()
    }

    /// View experimental settings section
    fn view_experimental_section(&self, i18n: &I18n) -> Element<'_, SettingsMessage> {
        column![
            text(i18n.t("Experimental")).size(18),
            Space::with_height(Length::Fixed(10.0)),
            checkbox(i18n.t("Use Wayland driver"), self.config.wayland_driver)
                .on_toggle(SettingsMessage::WaylandDriverToggled)
                .style(DeepSpace::checkbox),
            checkbox(i18n.t("Enable HDR"), self.config.enable_hdr)
                .on_toggle(SettingsMessage::EnableHdrToggled)
                .style(DeepSpace::checkbox),
            checkbox(i18n.t("Enable WOW64"), self.config.enable_wow64)
                .on_toggle(SettingsMessage::EnableWow64Toggled)
                .style(DeepSpace::checkbox),
        ]
        .spacing(5)
        .into()
    }

    /// View tools section
    fn view_tools_section(&self, i18n: &I18n) -> Element<'_, SettingsMessage> {
        column![
            text(i18n.t("Tools")).size(18),
            Space::with_height(Length::Fixed(10.0)),
            row![
                button(text(i18n.t("Proton Manager")).size(14))
                    .on_press(SettingsMessage::ProtonManagerClicked)
                    .style(DeepSpace::button),
                Space::with_width(Length::Fixed(10.0)),
                button(text("Winecfg").size(14))
                    .on_press(SettingsMessage::WinecfgClicked)
                    .style(DeepSpace::button),
                Space::with_width(Length::Fixed(10.0)),
                button(text("Winetricks").size(14))
                    .on_press(SettingsMessage::WinetricksClicked)
                    .style(DeepSpace::button),
                Space::with_width(Length::Fixed(10.0)),
                button(text(i18n.t("Run")).size(14))
                    .on_press(SettingsMessage::RunClicked)
                    .style(DeepSpace::button),
            ]
            .spacing(10),
        ]
        .spacing(5)
        .into()
    }

    /// View actions section
    fn view_actions_section(&self, i18n: &I18n) -> Element<'_, SettingsMessage> {
        column![
            text(i18n.t("Actions")).size(18),
            Space::with_height(Length::Fixed(10.0)),
            row![
                button(text(i18n.t("Backup")).size(14))
                    .on_press(SettingsMessage::BackupClicked)
                    .style(DeepSpace::button),
                Space::with_width(Length::Fixed(10.0)),
                button(text(i18n.t("Restore")).size(14))
                    .on_press(SettingsMessage::RestoreClicked)
                    .style(DeepSpace::button),
                Space::with_width(Length::Fixed(10.0)),
                button(text(i18n.t("Reset to Defaults")).size(14))
                    .on_press(SettingsMessage::ResetToDefaults)
                    .style(DeepSpace::button),
            ]
            .spacing(10),
            Space::with_height(Length::Fixed(10.0)),
            row![
                button(text(i18n.t("Show Logs")).size(14))
                    .on_press(SettingsMessage::ShowLogsClicked)
                    .style(DeepSpace::button),
                Space::with_width(Length::Fixed(10.0)),
                button(text(i18n.t("Clear Logs")).size(14))
                    .on_press(SettingsMessage::ClearLogsClicked)
                    .style(DeepSpace::button),
            ]
            .spacing(10),
        ]
        .spacing(5)
        .into()
    }

    /// View buttons section
    fn view_buttons(&self, i18n: &I18n) -> Element<'_, SettingsMessage> {
        row![
            Space::with_width(Length::Fill),
            button(text(i18n.t("Cancel")).size(14))
                .on_press(SettingsMessage::Cancel)
                .width(Length::Fixed(150.0))
                .style(DeepSpace::button),
            Space::with_width(Length::Fixed(10.0)),
            button(text(i18n.t("Apply")).size(14))
                .on_press(SettingsMessage::Confirm)
                .width(Length::Fixed(150.0))
                .style(DeepSpace::primary_button),
        ]
        .spacing(10)
        .into()
    }
}
