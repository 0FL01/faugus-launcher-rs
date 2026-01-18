// Main window implementation
// Primary GUI window for Faugus Launcher using Iced

use iced::widget::{
    button, column, container, horizontal_space, image, mouse_area, row, scrollable, text,
    text_input,
};
use iced::{Alignment, Element, Length, Task};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{error, info};

use crate::config::{AppConfig, Game, InterfaceMode};
use crate::gui::styles::DeepSpace;
use crate::icons::IconManager;
use crate::launcher::{GameLaunchController, LaunchStatus};
use crate::locale::I18n;
use crate::shortcuts::DesktopShortcutManager;
use crate::steam::SteamShortcuts;
use crate::Message;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Main application window
pub struct MainWindow {
    config: AppConfig,
    i18n: I18n,
    games: Vec<Game>,
    selected_game_index: Option<usize>,
    search_query: String,
    launch_controller: GameLaunchController,
    launch_status: HashMap<String, LaunchStatus>,
    icon_cache: HashMap<String, PathBuf>,
    last_click: Option<(usize, std::time::Instant)>,
    show_error_dialog: Option<String>,
}

impl MainWindow {
    /// Create a new main window
    pub fn new(config: AppConfig, i18n: I18n, games: Vec<Game>) -> Self {
        let launch_controller = GameLaunchController::new();
        let mut launch_status = HashMap::new();
        let mut icon_cache = HashMap::new();

        // Check initial launch status for all games
        for game in &games {
            launch_status.insert(
                game.title.clone(),
                launch_controller.get_status(&game.title),
            );
            // Cache icon paths
            let icon_path = IconManager::get_icon_path(&game.gameid);
            if icon_path.exists() {
                icon_cache.insert(game.gameid.clone(), icon_path);
            }
        }

        Self {
            config,
            i18n,
            games,
            selected_game_index: None,
            search_query: String::new(),
            launch_controller,
            launch_status,
            icon_cache,
            last_click: None,
            show_error_dialog: None,
        }
    }

    /// Get the config reference
    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    /// Get the i18n reference
    pub fn i18n(&self) -> &I18n {
        &self.i18n
    }

    /// Get the games list
    pub fn games(&self) -> &[Game] {
        &self.games
    }

    /// Get the selected game index
    pub fn selected_game_index(&self) -> Option<usize> {
        self.selected_game_index
    }

    /// Get the launch controller
    pub fn launch_controller(&self) -> &GameLaunchController {
        &self.launch_controller
    }

    /// Reload games from storage
    pub fn reload_games(&mut self) {
        self.games = Game::load_all().unwrap_or_default();

        // Update icon cache
        for game in &self.games {
            let icon_path = IconManager::get_icon_path(&game.gameid);
            if icon_path.exists() && !self.icon_cache.contains_key(&game.gameid) {
                self.icon_cache.insert(game.gameid.clone(), icon_path);
            }
        }
    }

    /// Update the configuration
    pub fn update_config(&mut self, config: AppConfig) {
        self.config = config;
    }

    /// Get the title for the window
    pub fn title(&self) -> String {
        format!("Faugus Launcher {}", VERSION)
    }

    /// Update launch status for a game
    pub fn update_launch_status(&mut self, title: &str, status: LaunchStatus) {
        self.launch_status.insert(title.to_string(), status);
    }

    /// Get icon path for a game (from cache or load)
    fn get_icon_path(&self, game_id: &str) -> Option<PathBuf> {
        self.icon_cache.get(game_id).cloned()
    }

    /// Load icon for display
    fn load_icon(&self, game_id: &str) -> Element<'_, Message> {
        if let Some(icon_path) = self.get_icon_path(game_id) {
            // Check if icon file exists
            if icon_path.exists() {
                return image(icon_path.clone())
                    .width(Length::Fixed(32.0))
                    .height(Length::Fixed(32.0))
                    .into();
            }
        }

        // Fallback: show placeholder text
        container(text("🎮").size(24))
            .width(Length::Fixed(32.0))
            .height(Length::Fixed(32.0))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .into()
    }

    /// Load medium icon for display (blocks mode)
    fn load_medium_icon(&self, game_id: &str) -> Element<'_, Message> {
        if let Some(icon_path) = self.get_icon_path(game_id) {
            if icon_path.exists() {
                return container(
                    image(icon_path.clone())
                        .width(Length::Fixed(64.0))
                        .height(Length::Fixed(64.0)),
                )
                .align_x(iced::alignment::Horizontal::Center)
                .into();
            }
        }

        container(text("🎮").size(48))
            .align_x(iced::alignment::Horizontal::Center)
            .into()
    }

    /// Load large icon for display (banners mode fallback)
    fn load_large_icon(&self, game_id: &str) -> Element<'_, Message> {
        if let Some(icon_path) = self.get_icon_path(game_id) {
            if icon_path.exists() {
                return container(
                    image(icon_path.clone())
                        .width(Length::Fixed(128.0))
                        .height(Length::Fixed(128.0)),
                )
                .align_x(iced::alignment::Horizontal::Center)
                .into();
            }
        }

        container(text("🎮").size(96))
            .align_x(iced::alignment::Horizontal::Center)
            .into()
    }

    /// Update the window state
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded => {
                info!("Application loaded");
                Task::none()
            }
            Message::GameSelected(index) => {
                self.selected_game_index = index;
                Task::none()
            }
            Message::GameClicked(index) => {
                // Check for double click
                let now = std::time::Instant::now();
                if let Some((last_index, last_time)) = self.last_click {
                    if last_index == index && now.duration_since(last_time).as_millis() < 500 {
                        self.last_click = None;
                        return Task::done(Message::GameDoubleClicked(index));
                    }
                }
                self.last_click = Some((index, now));
                self.selected_game_index = Some(index);
                Task::none()
            }
            Message::GameDoubleClicked(index) => {
                // Double click launches the game
                self.selected_game_index = Some(index);
                if let Some(game) = self.games.get(index).cloned() {
                    info!("Launching game via double-click: {}", game.title);
                    let task = self.launch_controller.launch_game(game);
                    return task.map(Message::LaunchMessage);
                }
                Task::none()
            }
            Message::PlayClicked => {
                if let Some(index) = self.selected_game_index {
                    if let Some(game) = self.games.get(index).cloned() {
                        info!("Launching game: {}", game.title);
                        let task = self.launch_controller.launch_game(game);
                        return task.map(Message::LaunchMessage);
                    }
                }
                Task::none()
            }
            Message::KillProcessClicked => {
                if let Some(index) = self.selected_game_index {
                    if let Some(game) = self.games.get(index) {
                        info!("Killing game process: {}", game.title);
                        if let Err(e) = self.launch_controller.terminate_game(&game.title) {
                            eprintln!("Failed to terminate game: {}", e);
                        }
                    }
                }
                Task::none()
            }
            Message::AddClicked => {
                info!("Add game clicked");
                Task::done(Message::ShowAddGameDialog)
            }
            Message::EditClicked => {
                if let Some(index) = self.selected_game_index {
                    info!(
                        "Edit game: {}",
                        self.games
                            .get(index)
                            .map(|g| g.title.as_str())
                            .unwrap_or("unknown")
                    );
                    Task::done(Message::ShowEditGameDialog(index))
                } else {
                    Task::none()
                }
            }
            Message::DeleteClicked => {
                // DeleteClicked is now handled in main.rs to show confirmation dialog
                Task::none()
            }
            Message::DeleteConfirmed(index, remove_prefix) => {
                if let Some(game) = self.games.get(index).cloned() {
                    info!(
                        "Deleting game: {} (remove_prefix: {})",
                        game.title, remove_prefix
                    );

                    // Delete game from storage
                    if let Err(e) = game.delete() {
                        error!("Failed to delete game: {}", e);
                    } else {
                        // Remove prefix folder if requested
                        if remove_prefix {
                            let prefix_path = &game.prefix;
                            let default_prefix = crate::config::paths::Paths::default_prefix();
                            // SAFETY: Never delete "default" prefix or the shared default prefix
                            if prefix_path != &default_prefix
                                && prefix_path.to_string_lossy() != "default"
                                && prefix_path.exists()
                            {
                                info!("Removing prefix folder: {:?}", prefix_path);
                                if let Err(e) = std::fs::remove_dir_all(prefix_path) {
                                    error!("Failed to remove prefix folder: {}", e);
                                }
                            }
                        }

                        // Remove from Steam shortcuts if present
                        if let Ok(mut shortcuts) = SteamShortcuts::load() {
                            if shortcuts.contains(&game.title) {
                                if let Err(e) = shortcuts.remove(&game.title) {
                                    error!("Failed to remove Steam shortcut: {}", e);
                                } else if let Err(e) = shortcuts.save() {
                                    error!("Failed to save Steam shortcuts: {}", e);
                                } else {
                                    info!("Steam shortcut removed for: {}", game.title);
                                }
                            }
                        }

                        // Remove desktop shortcuts if present
                        if DesktopShortcutManager::exists(&game) {
                            if let Err(e) = DesktopShortcutManager::remove(&game) {
                                error!("Failed to remove desktop shortcuts: {}", e);
                            } else {
                                info!("Desktop shortcuts removed for: {}", game.title);
                            }
                        }

                        // Remove icon if present
                        if let Err(e) = IconManager::delete_icon(&game.gameid) {
                            error!("Failed to remove icon: {}", e);
                        }

                        // Reload games list
                        self.reload_games();

                        // Clear selection
                        self.selected_game_index = None;
                    }
                }
                Task::none()
            }
            Message::SettingsClicked => {
                info!("Settings clicked");
                Task::done(Message::ShowSettingsDialog)
            }
            Message::SearchChanged(query) => {
                self.search_query = query;
                // TODO: Filter games based on query
                Task::none()
            }
            Message::Tick => {
                let dead_games = self.launch_controller.check_processes();
                for title in dead_games {
                    self.update_launch_status(&title, LaunchStatus::NotRunning);
                }
                Task::none()
            }
            Message::LaunchMessage(msg) => {
                use crate::launcher::LaunchMessage;
                match msg {
                    LaunchMessage::Launched(title, process) => {
                        info!("Game launched successfully: {}", title);
                        self.update_launch_status(&title, LaunchStatus::Running(process));
                        // Handle close on launch
                        if self.config.close_on_launch {
                            // Close application
                            // TODO: Implement proper close
                        }
                    }
                    LaunchMessage::LaunchFailed(title, error) => {
                        info!("Game launch failed: {} - {}", title, error);
                        self.update_launch_status(&title, LaunchStatus::Error(error.clone()));
                        self.show_error_dialog =
                            Some(format!("Failed to launch {}: {}", title, error));
                    }
                    LaunchMessage::ProcessExited(title, _pid) => {
                        info!("Game process exited: {}", title);
                        self.update_launch_status(&title, LaunchStatus::NotRunning);
                    }
                    LaunchMessage::Progress(title, msg) => {
                        info!("Launch progress for {}: {}", title, msg);
                    }
                }
                Task::none()
            }
            Message::ProcessExited(title) => {
                info!("Process exited: {}", title);
                self.launch_controller.on_process_exited(&title);
                self.update_launch_status(&title, LaunchStatus::NotRunning);
                Task::none()
            }
            Message::CloseErrorDialog => {
                self.show_error_dialog = None;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    /// View the window
    pub fn view(&self) -> Element<'_, Message> {
        let header = self.view_header();
        let content = self.view_content();
        let sidebar = self.view_sidebar();

        let main_content = row![sidebar, content].spacing(10);

        // Sidebar and Content layout
        let layout = container(column![header, main_content].spacing(10))
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fill);

        if let Some(error) = &self.show_error_dialog {
            let error_modal = container(
                column![
                    text("Error").size(20),
                    text(error),
                    button(text("Close"))
                        .on_press(Message::CloseErrorDialog)
                        .padding(10)
                ]
                .spacing(20)
                .align_x(Alignment::Center),
            )
            .width(Length::Fixed(400.0))
            .padding(20)
            .style(iced::widget::container::bordered_box);

            iced::widget::stack![
                layout,
                container(error_modal)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .style(|_theme| iced::widget::container::Style {
                        background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.7).into()), // Darker overlay for modal
                        ..Default::default()
                    })
            ]
            .into()
        } else {
            layout.into()
        }
    }

    /// View the header
    fn view_header(&self) -> Element<'_, Message> {
        let version_text = format!("Version: {}", VERSION);
        row![
            text(self.title()).size(24),
            horizontal_space(),
            text(version_text).size(12),
        ]
        .spacing(10)
        .padding(10)
        .align_y(Alignment::Center)
        .into()
    }

    /// View the content area (games list)
    fn view_content(&self) -> Element<'_, Message> {
        match self.config.interface_mode {
            InterfaceMode::List => self.view_list_mode(),
            InterfaceMode::Blocks => self.view_blocks_mode(),
            InterfaceMode::Banners => self.view_banners_mode(),
        }
    }

    /// View in list mode
    fn view_list_mode(&self) -> Element<'_, Message> {
        let games_list: Vec<Element<Message>> = self
            .games
            .iter()
            .filter(|game| self.game_matches_search(game))
            .enumerate()
            .map(|(index, game)| {
                let is_selected = self.selected_game_index == Some(index);
                let is_hidden = game.hidden;

                // Load icon for this game
                let icon = self.load_icon(&game.gameid);

                // Dim the title if hidden
                let title_text = if is_hidden {
                    text(&game.title).size(16).style(|_theme: &iced::Theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6)),
                            ..Default::default()
                        }
                    })
                } else {
                    text(&game.title).size(16)
                };

                let game_row = row![icon, title_text, horizontal_space(),]
                    .spacing(10)
                    .padding(10)
                    .align_y(Alignment::Center);

                let container =
                    container(game_row)
                        .padding(10)
                        .width(Length::Fill)
                        .style(if is_selected {
                            DeepSpace::container
                        } else {
                            DeepSpace::transparent_container
                        });

                // Wrap in mouse_area for click handling
                mouse_area(container)
                    .on_press(Message::GameClicked(index))
                    .on_right_press(Message::GameRightClicked(index))
                    .into()
            })
            .collect();

        scrollable(column(games_list).spacing(5))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// View in blocks mode
    fn view_blocks_mode(&self) -> Element<'_, Message> {
        let games_blocks: Vec<Element<Message>> =
            self.games
                .iter()
                .filter(|game| self.game_matches_search(game))
                .enumerate()
                .map(|(index, game)| {
                    let is_selected = self.selected_game_index == Some(index);
                    let is_hidden = game.hidden;

                    // Load icon for this game (larger size for blocks)
                    let icon = if let Some(ref banner_path) = game.banner {
                        if banner_path.exists() {
                            Element::from(
                                container(
                                    image(iced::widget::image::Handle::from_path(banner_path))
                                        .width(Length::Fixed(180.0))
                                        .height(Length::Fixed(90.0)),
                                )
                                .align_x(iced::alignment::Horizontal::Center),
                            )
                        } else {
                            self.load_medium_icon(&game.gameid)
                        }
                    } else {
                        self.load_medium_icon(&game.gameid)
                    };

                    // Dim the title if hidden
                    let title_text = if is_hidden {
                        text(&game.title).size(14).style(|_theme: &iced::Theme| {
                            iced::widget::text::Style {
                                color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6)),
                                ..Default::default()
                            }
                        })
                    } else {
                        text(&game.title).size(14)
                    };

                    let content = column![icon, title_text]
                        .spacing(5)
                        .align_x(Alignment::Center);

                    let container = container(content).padding(10).width(200).height(180).style(
                        if is_selected {
                            DeepSpace::container
                        } else {
                            DeepSpace::transparent_container
                        },
                    );

                    mouse_area(container)
                        .on_press(Message::GameClicked(index))
                        .into()
                })
                .collect();

        scrollable(row(games_blocks).spacing(10))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// View in banners mode
    fn view_banners_mode(&self) -> Element<'_, Message> {
        let games_banners: Vec<Element<Message>> = self
            .games
            .iter()
            .filter(|game| self.game_matches_search(game))
            .enumerate()
            .map(|(index, game)| {
                let is_selected = self.selected_game_index == Some(index);
                let is_hidden = game.hidden;

                // Load large icon for banner
                let icon = if let Some(ref banner_path) = game.banner {
                    if banner_path.exists() {
                        Element::from(
                            container(
                                image(iced::widget::image::Handle::from_path(banner_path))
                                    .width(Length::Fixed(440.0))
                                    .height(Length::Fixed(220.0)),
                            )
                            .align_x(iced::alignment::Horizontal::Center),
                        )
                    } else {
                        self.load_large_icon(&game.gameid)
                    }
                } else {
                    self.load_large_icon(&game.gameid)
                };

                // Dim the title if hidden
                let title_text = if is_hidden {
                    text(&game.title).size(16).style(|_theme: &iced::Theme| {
                        iced::widget::text::Style {
                            color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6)),
                            ..Default::default()
                        }
                    })
                } else {
                    text(&game.title).size(16)
                };

                let content = column![icon, title_text]
                    .spacing(5)
                    .align_x(Alignment::Center);

                let banner_height = if self.config.smaller_banners {
                    215
                } else {
                    322
                };

                let container = container(content)
                    .padding(10)
                    .width(460)
                    .height(banner_height)
                    .style(if is_selected {
                        DeepSpace::container
                    } else {
                        DeepSpace::transparent_container
                    });

                mouse_area(container)
                    .on_press(Message::GameClicked(index))
                    .into()
            })
            .collect();

        scrollable(row(games_banners).spacing(10))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    /// View the sidebar
    fn view_sidebar(&self) -> Element<'_, Message> {
        let search = text_input(&self.i18n.t("Search games..."), &self.search_query)
            .on_input(Message::SearchChanged)
            .padding(10)
            .width(Length::Fill)
            .style(DeepSpace::text_input);

        // Get selected game status
        let selected_status = self
            .selected_game_index
            .and_then(|index| self.games.get(index))
            .and_then(|game| self.launch_status.get(&game.title))
            .cloned()
            .unwrap_or(LaunchStatus::NotRunning);

        let is_running = matches!(
            selected_status,
            LaunchStatus::Running(_) | LaunchStatus::Launching
        );
        let has_error = matches!(selected_status, LaunchStatus::Error(_));

        // Play/Kill button text
        let play_kill_text = if is_running {
            self.i18n.t("Stop")
        } else if has_error {
            self.i18n.t("Retry")
        } else {
            self.i18n.t("Play")
        };

        let play_kill_button = button(text(play_kill_text))
            .on_press(if is_running {
                Message::KillProcessClicked
            } else {
                Message::PlayClicked
            })
            .padding(10)
            .width(Length::Fill)
            .style(DeepSpace::button);

        let status_text = match selected_status {
            LaunchStatus::Running(_) => {
                text("Running")
                    .size(11)
                    .style(|_| iced::widget::text::Style {
                        color: Some(crate::gui::styles::colors::ACCENT),
                        ..Default::default()
                    })
            }
            LaunchStatus::Launching => {
                text("Launching...")
                    .size(11)
                    .style(|_| iced::widget::text::Style {
                        color: Some(crate::gui::styles::colors::ACCENT),
                        ..Default::default()
                    })
            }
            LaunchStatus::Error(ref e) => {
                text(format!("Error: {}", e))
                    .size(11)
                    .style(|_| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(1.0, 0.3, 0.3)),
                        ..Default::default()
                    })
            }
            LaunchStatus::NotRunning => text("").size(11),
        };

        let add_button = button(text(self.i18n.t("Add")))
            .on_press(Message::AddClicked)
            .padding(10)
            .width(Length::Fill)
            .style(DeepSpace::menu_button);

        let settings_button = button(text(self.i18n.t("Settings")))
            .on_press(Message::SettingsClicked)
            .padding(10)
            .width(Length::Fill)
            .style(DeepSpace::menu_button);

        let kill_all_button = button(text(self.i18n.t("Kill All")))
            .on_press(Message::KillAllProcesses)
            .padding(10)
            .width(Length::Fill)
            .style(DeepSpace::menu_button);

        column![
            search,
            play_kill_button,
            status_text,
            add_button,
            settings_button,
            kill_all_button,
        ]
        .spacing(10)
        .width(200)
        .into()
    }

    /// Filter games based on search query and hidden state
    fn game_matches_search(&self, game: &Game) -> bool {
        // Check if game matches search query
        let matches_query = if self.search_query.is_empty() {
            true
        } else {
            let query = self.search_query.to_lowercase();
            game.title.to_lowercase().contains(&query)
        };

        // Check if hidden games should be shown
        let matches_hidden = if game.hidden {
            self.config.show_hidden
        } else {
            true
        };

        matches_query && matches_hidden
    }

    /// Subscribe to events
    pub fn subscription(&self) -> Task<Message> {
        // TODO: Add subscriptions for process monitoring, IPC, etc.
        Task::none()
    }
}

impl Default for MainWindow {
    fn default() -> Self {
        Self::new(AppConfig::default(), I18n::default(), Vec::new())
    }
}
