// Add Game Dialog
// Dialog for adding or editing games in Faugus Launcher

use iced::widget::{
    button, checkbox, column, container, pick_list, row, scrollable, text, text_input, Space,
};
use iced::{Element, Length, Task};
use std::fmt;
use std::path::PathBuf;

use crate::config::{AppConfig, Game};
use crate::gui::file_picker;
use crate::locale::I18n;

/// Launcher types supported by Faugus Launcher
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LauncherType {
    #[default]
    Steam,
    Heroic,
    Lutris,
    Epic,
    Gog,
    Other,
}

impl LauncherType {
    pub const ALL: [LauncherType; 6] = [
        LauncherType::Steam,
        LauncherType::Heroic,
        LauncherType::Lutris,
        LauncherType::Epic,
        LauncherType::Gog,
        LauncherType::Other,
    ];
}

impl fmt::Display for LauncherType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LauncherType::Steam => "Steam",
                LauncherType::Heroic => "Heroic",
                LauncherType::Lutris => "Lutris",
                LauncherType::Epic => "Epic",
                LauncherType::Gog => "GOG",
                LauncherType::Other => "Other",
            }
        )
    }
}

/// Messages for the Add Game dialog
#[derive(Debug, Clone)]
pub enum AddGameMessage {
    /// Title changed
    TitleChanged(String),
    /// Path changed
    PathChanged(String),
    /// Prefix changed
    PrefixChanged(String),
    /// Launcher type changed
    LauncherTypeChanged(LauncherType),
    /// Runner changed
    RunnerChanged(String),
    /// Protonfix (UMU ID) changed
    ProtonfixChanged(String),
    /// Launch arguments changed
    LaunchArgumentsChanged(String),
    /// Game arguments changed
    GameArgumentsChanged(String),
    /// Additional application path changed
    AddAppChanged(String),
    /// MangoHud checkbox toggled
    MangoHudToggled(bool),
    /// GameMode checkbox toggled
    GameModeToggled(bool),
    /// Disable Hidraw checkbox toggled
    DisableHidrawToggled(bool),
    /// Create desktop shortcut toggled
    ShortcutDesktopToggled(bool),
    /// Create app menu shortcut toggled
    ShortcutAppMenuToggled(bool),
    /// Create Steam shortcut toggled
    ShortcutSteamToggled(bool),
    /// Browse for game executable
    BrowsePath,
    /// Game executable picked
    PathPicked(Option<PathBuf>),
    /// Browse for prefix
    BrowsePrefix,
    /// Prefix picked
    PrefixPicked(Option<PathBuf>),
    /// Browse for additional app
    BrowseAddApp,
    /// Additional app picked
    AddAppPicked(Option<PathBuf>),
    /// Open Protonfix URL in browser
    OpenProtonfixUrl,
    /// Lossless Scaling button clicked
    LosslessClicked,
    /// Confirm dialog
    Confirm,
    /// Cancel dialog
    Cancel,
}

/// State for the Add Game dialog
#[derive(Debug, Clone)]
pub struct AddGameDialog {
    /// Whether this is an edit operation (vs add new)
    is_edit: bool,
    /// Game being edited (None for new game)
    editing_game: Option<Game>,
    /// Dialog title
    title: String,

    // Form fields
    /// Game title
    game_title: String,
    /// Game executable path
    game_path: PathBuf,
    /// Wine prefix path
    prefix: PathBuf,
    /// Launcher type
    launcher_type: LauncherType,
    /// Selected runner index
    runner_index: usize,
    /// Available runners list
    runners: Vec<String>,
    /// UMU ID (Protonfix)
    protonfix: String,
    /// Launch arguments (environment variables, pre-launch commands)
    launch_arguments: String,
    /// Game arguments (passed to the game executable)
    game_arguments: String,
    /// Additional application path
    addapp_path: PathBuf,

    // Checkboxes
    /// Enable MangoHud
    mangohud: bool,
    /// Enable GameMode
    gamemode: bool,
    /// Disable HIDRAW (for controller support)
    disable_hidraw: bool,
    /// Create desktop shortcut
    shortcut_desktop: bool,
    /// Create app menu shortcut
    shortcut_appmenu: bool,
    /// Create Steam shortcut
    shortcut_steam: bool,

    // Lossless Scaling settings
    /// Lossless Scaling enabled
    lossless_enabled: bool,
    /// Lossless Scaling frame multiplier
    lossless_multiplier: u32,
    /// Lossless Scaling flow scale
    lossless_flow: bool,
    /// Lossless Scaling performance mode
    lossless_performance: bool,
    /// Lossless Scaling HDR mode
    lossless_hdr: bool,

    /// Validation error message (if any)
    error_message: Option<String>,
    /// Dialog is showing Lossless sub-dialog
    showing_lossless: bool,
}

impl AddGameDialog {
    /// Create a new Add Game dialog (for adding a new game)
    pub fn new(config: &AppConfig, i18n: &I18n) -> Self {
        let runners = Self::get_available_runners();
        let default_runner_index = runners
            .iter()
            .position(|r| r == &config.default_runner)
            .unwrap_or(0);

        Self {
            is_edit: false,
            editing_game: None,
            title: i18n.t("New Game/App"),

            game_title: String::new(),
            game_path: PathBuf::new(),
            prefix: config.default_prefix.clone(),
            launcher_type: LauncherType::default(),
            runner_index: default_runner_index,
            runners,
            protonfix: String::new(),
            launch_arguments: String::new(),
            game_arguments: String::new(),
            addapp_path: PathBuf::new(),

            mangohud: config.mangohud,
            gamemode: config.gamemode,
            disable_hidraw: config.disable_hidraw,
            shortcut_desktop: false,
            shortcut_appmenu: false,
            shortcut_steam: false,

            lossless_enabled: false,
            lossless_multiplier: 2,
            lossless_flow: false,
            lossless_performance: false,
            lossless_hdr: false,

            error_message: None,
            showing_lossless: false,
        }
    }

    /// Create an Edit Game dialog (for editing an existing game)
    pub fn edit(game: Game, config: &AppConfig, i18n: &I18n) -> Self {
        let mut dialog = Self::new(config, i18n);
        dialog.is_edit = true;
        dialog.editing_game = Some(game.clone());
        dialog.title = i18n.t("Edit Game/App");

        // Load game data into form
        dialog.game_title = game.title;
        dialog.game_path = game.path;
        dialog.prefix = game.prefix;
        dialog.launcher_type = LauncherType::default(); // Default for now
        dialog.protonfix = game.protonfix;
        dialog.launch_arguments = game.launch_arguments;
        dialog.game_arguments = game.game_arguments;
        dialog.mangohud = game.mangohud;
        dialog.gamemode = game.gamemode;
        dialog.disable_hidraw = game.disable_hidraw;
        dialog.addapp_path = if game.addapp_checkbox {
            PathBuf::from(&game.addapp)
        } else {
            PathBuf::new()
        };

        // Load runner
        let runners = Self::get_available_runners();
        dialog.runners = runners;
        dialog.runner_index = dialog
            .runners
            .iter()
            .position(|r| r == &game.runner)
            .unwrap_or(0);

        // Load lossless settings
        dialog.lossless_enabled = game.lossless_enabled;
        dialog.lossless_multiplier = game.lossless_multiplier;
        dialog.lossless_flow = game.lossless_flow;
        dialog.lossless_performance = game.lossless_performance;
        dialog.lossless_hdr = game.lossless_hdr;

        // Load shortcut settings from addapp fields
        dialog.shortcut_desktop = game.addapp.contains("desktop");
        dialog.shortcut_appmenu = game.addapp.contains("appmenu");
        dialog.shortcut_steam = game.addapp.contains("steam");

        dialog
    }

    /// Get list of available Proton runners
    fn get_available_runners() -> Vec<String> {
        vec![
            "UMU-Proton Latest".to_string(),
            "GE-Proton Latest (default)".to_string(),
            "Proton-GE Latest".to_string(),
        ]
    }

    /// Update the dialog state
    pub fn update(&mut self, message: AddGameMessage) -> Task<AddGameMessage> {
        match message {
            AddGameMessage::TitleChanged(title) => {
                self.game_title = title;
                // Auto-update prefix based on title
                if self.game_title.is_empty()
                    || self.prefix == Self::default_prefix_for_game(&self.game_title)
                {
                    self.prefix = Self::default_prefix_for_game(&self.game_title);
                }
                self.error_message = None;
            }
            AddGameMessage::PathChanged(path) => {
                self.game_path = PathBuf::from(path);
                self.error_message = None;
            }
            AddGameMessage::PrefixChanged(prefix) => {
                self.prefix = PathBuf::from(prefix);
                self.error_message = None;
            }
            AddGameMessage::LauncherTypeChanged(launcher_type) => {
                self.launcher_type = launcher_type;
            }
            AddGameMessage::RunnerChanged(runner) => {
                if let Some(index) = self.runners.iter().position(|r| r == &runner) {
                    self.runner_index = index;
                }
            }
            AddGameMessage::ProtonfixChanged(protonfix) => {
                self.protonfix = protonfix;
            }
            AddGameMessage::LaunchArgumentsChanged(args) => {
                self.launch_arguments = args;
            }
            AddGameMessage::GameArgumentsChanged(args) => {
                self.game_arguments = args;
            }
            AddGameMessage::AddAppChanged(path) => {
                self.addapp_path = PathBuf::from(path);
            }
            AddGameMessage::MangoHudToggled(enabled) => {
                self.mangohud = enabled;
            }
            AddGameMessage::GameModeToggled(enabled) => {
                self.gamemode = enabled;
            }
            AddGameMessage::DisableHidrawToggled(enabled) => {
                self.disable_hidraw = enabled;
            }
            AddGameMessage::ShortcutDesktopToggled(enabled) => {
                self.shortcut_desktop = enabled;
            }
            AddGameMessage::ShortcutAppMenuToggled(enabled) => {
                self.shortcut_appmenu = enabled;
            }
            AddGameMessage::ShortcutSteamToggled(enabled) => {
                self.shortcut_steam = enabled;
            }
            AddGameMessage::BrowsePath => {
                return Task::perform(file_picker::pick_file(), AddGameMessage::PathPicked);
            }
            AddGameMessage::PathPicked(path) => {
                if let Some(path) = path {
                    self.game_path = path;
                }
            }
            AddGameMessage::BrowsePrefix => {
                return Task::perform(file_picker::pick_folder(), AddGameMessage::PrefixPicked);
            }
            AddGameMessage::PrefixPicked(path) => {
                if let Some(path) = path {
                    self.prefix = path;
                }
            }
            AddGameMessage::BrowseAddApp => {
                return Task::perform(file_picker::pick_file(), AddGameMessage::AddAppPicked);
            }
            AddGameMessage::AddAppPicked(path) => {
                if let Some(path) = path {
                    self.addapp_path = path;
                }
            }
            AddGameMessage::OpenProtonfixUrl => {
                // TODO: Open browser with UMU ID URL
                tracing::info!("Open Protonfix URL: https://umu.openwinecomponents.org/");
            }
            AddGameMessage::LosslessClicked => {
                self.showing_lossless = !self.showing_lossless;
            }
            AddGameMessage::Confirm => {
                if self.validate() {
                    return Task::done(AddGameMessage::Confirm);
                }
            }
            AddGameMessage::Cancel => {
                return Task::done(AddGameMessage::Cancel);
            }
        }
        Task::none()
    }

    /// Get default prefix path for a game
    fn default_prefix_for_game(title: &str) -> PathBuf {
        let formatted_title = Self::format_title(title);
        let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
        PathBuf::from(home).join("Faugus").join(formatted_title)
    }

    /// Format game title for use in paths
    fn format_title(title: &str) -> String {
        title
            .trim()
            .to_lowercase()
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' {
                    c
                } else {
                    '-'
                }
            })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-")
    }

    /// Validate the form
    pub fn validate(&mut self) -> bool {
        // Check required fields
        if self.game_title.trim().is_empty() {
            self.error_message = Some("Title is required".to_string());
            return false;
        }

        if self.game_path.as_os_str().is_empty() || !self.game_path.exists() {
            self.error_message = Some("Path must be a valid executable".to_string());
            return false;
        }

        if self.prefix.as_os_str().is_empty() {
            self.error_message = Some("Prefix is required".to_string());
            return false;
        }

        self.error_message = None;
        true
    }

    /// Get the Steam shortcut checkbox state
    pub fn shortcut_steam(&self) -> bool {
        self.shortcut_steam
    }

    /// Get the desktop shortcut checkbox state
    pub fn shortcut_desktop(&self) -> bool {
        self.shortcut_desktop
    }

    /// Get the app menu shortcut checkbox state
    pub fn shortcut_appmenu(&self) -> bool {
        self.shortcut_appmenu
    }

    /// Get the game data from the form
    pub fn get_game(&self) -> Game {
        let gameid = if let Some(ref game) = self.editing_game {
            game.gameid.clone()
        } else {
            // Generate new game ID
            uuid::Uuid::new_v4().to_string()
        };

        // Build addapp string from shortcut checkboxes
        let mut addapp_parts = Vec::new();
        if self.shortcut_desktop {
            addapp_parts.push("desktop");
        }
        if self.shortcut_appmenu {
            addapp_parts.push("appmenu");
        }
        if self.shortcut_steam {
            addapp_parts.push("steam");
        }

        let addapp = if !self.addapp_path.as_os_str().is_empty() {
            self.addapp_path.to_string_lossy().to_string()
        } else if !addapp_parts.is_empty() {
            addapp_parts.join(",")
        } else {
            String::new()
        };

        let addapp_bat = if !addapp.is_empty() {
            format!(
                "{}/faugus-{}.bat",
                self.game_path
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("/"))
                    .display(),
                Self::format_title(&self.game_title)
            )
        } else {
            String::new()
        };

        let addapp_checkbox = !addapp.is_empty();

        Game {
            gameid,
            title: self.game_title.clone(),
            path: self.game_path.clone(),
            prefix: self.prefix.clone(),
            launch_arguments: self.launch_arguments.clone(),
            game_arguments: self.game_arguments.clone(),
            mangohud: self.mangohud,
            gamemode: self.gamemode,
            disable_hidraw: self.disable_hidraw,
            protonfix: self.protonfix.clone(),
            runner: self
                .runners
                .get(self.runner_index)
                .cloned()
                .unwrap_or_default(),
            addapp_checkbox,
            addapp,
            addapp_bat,
            banner: None, // TODO: Implement banner selection
            lossless_enabled: self.lossless_enabled,
            lossless_multiplier: self.lossless_multiplier,
            lossless_flow: self.lossless_flow,
            lossless_performance: self.lossless_performance,
            lossless_hdr: self.lossless_hdr,
            playtime: self.editing_game.as_ref().map(|g| g.playtime).unwrap_or(0),
            hidden: self
                .editing_game
                .as_ref()
                .map(|g| g.hidden)
                .unwrap_or(false),
        }
    }

    /// View the dialog
    pub fn view(&self, i18n: &I18n) -> Element<'_, AddGameMessage> {
        let content = if self.showing_lossless {
            self.view_lossless_dialog(i18n)
        } else {
            self.view_main_dialog(i18n)
        };

        container(content)
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// View the main dialog
    fn view_main_dialog(&self, i18n: &I18n) -> Element<'_, AddGameMessage> {
        let title_section = self.view_title_section(i18n);
        let path_section = self.view_path_section(i18n);
        let prefix_section = self.view_prefix_section(i18n);
        let launcher_type_section = self.view_launcher_type_section(i18n);
        let runner_section = self.view_runner_section(i18n);
        let protonfix_section = self.view_protonfix_section(i18n);
        let arguments_section = self.view_arguments_section(i18n);
        let options_section = self.view_options_section(i18n);
        let shortcuts_section = self.view_shortcuts_section(i18n);
        let buttons_section = self.view_buttons(i18n);

        let scrollable = scrollable(column![
            title_section,
            Space::with_height(Length::Fixed(10.0)),
            path_section,
            Space::with_height(Length::Fixed(10.0)),
            prefix_section,
            Space::with_height(Length::Fixed(10.0)),
            launcher_type_section,
            Space::with_height(Length::Fixed(10.0)),
            runner_section,
            Space::with_height(Length::Fixed(10.0)),
            protonfix_section,
            Space::with_height(Length::Fixed(10.0)),
            arguments_section,
            Space::with_height(Length::Fixed(10.0)),
            options_section,
            Space::with_height(Length::Fixed(10.0)),
            shortcuts_section,
        ])
        .width(Length::Fill);

        column![
            scrollable,
            Space::with_height(Length::Fixed(10.0)),
            buttons_section
        ]
        .spacing(10)
        .into()
    }

    /// View the title section
    fn view_title_section(&self, i18n: &I18n) -> Element<'_, AddGameMessage> {
        column![
            text(i18n.t("Title")).size(14),
            Space::with_height(Length::Fixed(5.0)),
            text_input(&i18n.t("Game Title"), &self.game_title)
                .on_input(AddGameMessage::TitleChanged),
        ]
        .spacing(5)
        .into()
    }

    /// View the path section
    fn view_path_section(&self, i18n: &I18n) -> Element<'_, AddGameMessage> {
        let path_display = self.game_path.display().to_string();
        column![
            text(i18n.t("Path")).size(14),
            Space::with_height(Length::Fixed(5.0)),
            row![
                text_input("/path/to/the/exe", &path_display).on_input(AddGameMessage::PathChanged),
                button(text("..."))
                    .on_press(AddGameMessage::BrowsePath)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(5),
        ]
        .spacing(5)
        .into()
    }

    /// View the prefix section
    fn view_prefix_section(&self, i18n: &I18n) -> Element<'_, AddGameMessage> {
        let prefix_display = self.prefix.display().to_string();
        column![
            text(i18n.t("Prefix")).size(14),
            Space::with_height(Length::Fixed(5.0)),
            row![
                text_input("/path/to/the/prefix", &prefix_display)
                    .on_input(AddGameMessage::PrefixChanged),
                button(text("..."))
                    .on_press(AddGameMessage::BrowsePrefix)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(5),
        ]
        .spacing(5)
        .into()
    }

    /// View the launcher type section
    fn view_launcher_type_section(&self, i18n: &I18n) -> Element<'_, AddGameMessage> {
        column![
            text(i18n.t("Launcher")).size(14),
            Space::with_height(Length::Fixed(5.0)),
            pick_list(
                &LauncherType::ALL[..],
                Some(self.launcher_type),
                AddGameMessage::LauncherTypeChanged
            )
            .width(Length::Fill),
        ]
        .spacing(5)
        .into()
    }

    /// View the runner section
    fn view_runner_section(&self, i18n: &I18n) -> Element<'_, AddGameMessage> {
        column![
            text(i18n.t("Proton")).size(14),
            Space::with_height(Length::Fixed(5.0)),
            pick_list(
                &self.runners[..],
                self.runners.get(self.runner_index).cloned(),
                AddGameMessage::RunnerChanged
            )
            .width(Length::Fill),
        ]
        .spacing(5)
        .into()
    }

    /// View the protonfix section
    fn view_protonfix_section(&self, _i18n: &I18n) -> Element<'_, AddGameMessage> {
        column![
            text("Protonfix (UMU ID)").size(14),
            Space::with_height(Length::Fixed(5.0)),
            row![
                text_input("UMU ID", &self.protonfix).on_input(AddGameMessage::ProtonfixChanged),
                button(text("..."))
                    .on_press(AddGameMessage::OpenProtonfixUrl)
                    .width(Length::Fixed(50.0)),
            ]
            .spacing(5),
        ]
        .spacing(5)
        .into()
    }

    /// View the arguments section
    fn view_arguments_section(&self, i18n: &I18n) -> Element<'_, AddGameMessage> {
        column![
            column![
                text(i18n.t("Launch Arguments")).size(14),
                Space::with_height(Length::Fixed(5.0)),
                text_input(
                    "e.g.: PROTON_USE_WINED3D=1 gamescope -W 2560 -H 1440",
                    &self.launch_arguments
                )
                .on_input(AddGameMessage::LaunchArgumentsChanged),
            ]
            .spacing(5),
            column![
                text(i18n.t("Game Arguments")).size(14),
                Space::with_height(Length::Fixed(5.0)),
                text_input("e.g.: -d3d11 -fullscreen", &self.game_arguments)
                    .on_input(AddGameMessage::GameArgumentsChanged),
            ]
            .spacing(5),
        ]
        .spacing(10)
        .into()
    }

    /// View the options section
    fn view_options_section(&self, i18n: &I18n) -> Element<'_, AddGameMessage> {
        column![
            text(i18n.t("Options")).size(14),
            Space::with_height(Length::Fixed(5.0)),
            checkbox("MangoHud", self.mangohud).on_toggle(AddGameMessage::MangoHudToggled),
            checkbox("GameMode", self.gamemode).on_toggle(AddGameMessage::GameModeToggled),
            checkbox(i18n.t("Disable Hidraw"), self.disable_hidraw)
                .on_toggle(AddGameMessage::DisableHidrawToggled),
            row![button(text(i18n.t("Lossless Scaling Frame Generation")))
                .on_press(AddGameMessage::LosslessClicked),]
            .padding(5),
        ]
        .spacing(5)
        .into()
    }

    /// View the shortcuts section
    fn view_shortcuts_section(&self, i18n: &I18n) -> Element<'_, AddGameMessage> {
        column![
            text(i18n.t("Shortcut")).size(14),
            Space::with_height(Length::Fixed(5.0)),
            checkbox(i18n.t("Desktop"), self.shortcut_desktop)
                .on_toggle(AddGameMessage::ShortcutDesktopToggled),
            checkbox(i18n.t("App Menu"), self.shortcut_appmenu)
                .on_toggle(AddGameMessage::ShortcutAppMenuToggled),
            checkbox("Steam", self.shortcut_steam).on_toggle(AddGameMessage::ShortcutSteamToggled),
        ]
        .spacing(5)
        .into()
    }

    /// View the buttons section
    fn view_buttons(&self, i18n: &I18n) -> Element<'_, AddGameMessage> {
        // Show error if any
        let error: Element<'_, AddGameMessage> = if let Some(ref error) = self.error_message {
            column![
                Space::with_height(Length::Fixed(5.0)),
                text(error)
                    .size(12)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color::new(1.0, 0.0, 0.0, 1.0)),
                        ..Default::default()
                    }),
            ]
            .into()
        } else {
            column![].into()
        };

        row![
            Space::with_width(Length::Fill),
            column![
                error,
                row![
                    button(text(i18n.t("Cancel")).size(14))
                        .on_press(AddGameMessage::Cancel)
                        .width(Length::Fixed(150.0)),
                    Space::with_width(Length::Fixed(10.0)),
                    button(text(i18n.t("Ok")).size(14))
                        .on_press(AddGameMessage::Confirm)
                        .width(Length::Fixed(150.0)),
                ]
                .spacing(10)
            ]
        ]
        .spacing(10)
        .into()
    }

    /// View the Lossless Scaling dialog
    fn view_lossless_dialog(&self, i18n: &I18n) -> Element<'_, AddGameMessage> {
        column![
            text(i18n.t("Lossless Scaling Frame Generation")).size(18),
            Space::with_height(Length::Fixed(20.0)),
            checkbox("Enable", self.lossless_enabled),
            text(i18n.t("Multiplier")).size(14),
            // TODO: Add spin button for multiplier
            text(format!("{}", self.lossless_multiplier)).size(14),
            text(i18n.t("Flow Scale")).size(14),
            // TODO: Add slider for flow scale
            text(format!("{}", self.lossless_flow)).size(14),
            checkbox(i18n.t("Performance Mode"), self.lossless_performance),
            checkbox(i18n.t("HDR Mode"), self.lossless_hdr),
            Space::with_height(Length::Fixed(20.0)),
            row![
                button(text(i18n.t("Cancel")).size(14)).on_press(AddGameMessage::LosslessClicked),
                Space::with_width(Length::Fixed(10.0)),
                button(text(i18n.t("Ok")).size(14)).on_press(AddGameMessage::LosslessClicked),
            ]
            .spacing(10),
        ]
        .spacing(10)
        .into()
    }
}
