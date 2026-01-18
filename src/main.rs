// Faugus Launcher - Rust Rewrite
// A lightweight launcher for Windows games on Linux using Iced

mod config;
mod gui;
mod icons;
mod launcher;
mod locale;
mod proton;
mod shortcuts;
mod steam;
mod tray;
mod utils;

use iced::widget::{container, mouse_area, stack, Space};
use iced::{window, Color, Element, Length, Padding, Point, Size, Subscription, Task};
use tracing::{error, info, warn};

use config::app_config::AppConfig;
use config::game_config::Game;
use gui::add_game_dialog::{AddGameDialog, AddGameMessage};
use gui::confirmation_dialog::ConfirmationDialog;
use gui::context_menu::{ContextMenu, ContextMenuMessage};
use gui::log_viewer_dialog::{LogViewerDialog, LogViewerMessage};
use gui::main_window::MainWindow;
use gui::proton_manager_dialog::{ProtonManagerDialog, ProtonManagerMessage};
use gui::settings_dialog::{SettingsDialog, SettingsMessage};
use icons::IconManager;
use launcher::LaunchMessage;
use locale::i18n::I18n;
use shortcuts::DesktopShortcutManager;
use shortcuts::ShortcutLocation;
use steam::SteamShortcuts;
use tray::{SystemTray, TrayConfig, TrayEvent};

use launcher::wine_tools;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application messages
#[derive(Debug, Clone)]
pub enum Message {
    Loaded,
    GameSelected(Option<usize>),
    GameClicked(usize),
    GameDoubleClicked(usize),
    PlayClicked,
    AddClicked,
    EditClicked,
    DeleteClicked,
    DeleteConfirmed(usize, bool),
    HideShowClicked,
    DuplicateClicked,
    KillProcessClicked,
    KillAllProcesses,
    SettingsClicked,
    SearchChanged(String),
    Tick,
    ProcessExited(String),
    LaunchMessage(LaunchMessage),
    ConfigUpdated(Result<(), String>),
    GameAdded(Result<Game, String>),
    GameUpdated(Result<Game, String>),
    GameDeleted(Result<(), String>),
    // Add Game Dialog messages
    AddGameDialog(AddGameMessage),
    ShowAddGameDialog,
    ShowEditGameDialog(usize),
    CloseAddGameDialog,
    // Settings Dialog messages
    SettingsDialog(SettingsMessage),
    ShowSettingsDialog,
    CloseSettingsDialog,
    // Log Viewer Dialog messages
    LogViewerDialog(LogViewerMessage),
    ShowLogViewerDialog,
    CloseLogViewerDialog,
    // Proton Manager Dialog messages
    ProtonManagerDialog(ProtonManagerMessage),
    ShowProtonManagerDialog,
    CloseProtonManagerDialog,
    // System Tray messages
    TrayEvent(TrayEvent),
    // Confirmation Dialog
    ShowConfirmationDialog(Box<ConfirmationDialog>),
    ConfirmationDialogClosed(bool),
    ToggleRemovePrefix,
    // Context Menu
    GameRightClicked(usize),
    ContextMenu(ContextMenuMessage),
    CloseContextMenu,
    MouseMoved(Point),
    CloseDialog,
    CloseErrorDialog,
    NoOp,
}

use gui::DialogState;

/// Main application state
pub struct FaugusLauncher {
    main_window: MainWindow,
    dialog: DialogState,
    pending_delete_index: Option<usize>,
    system_tray: Option<SystemTray>,
    mouse_position: Point,
}

impl FaugusLauncher {
    fn new() -> (Self, Task<Message>) {
        info!("Faugus Launcher {}", VERSION);

        // Initialize tracing
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();

        // Load configuration
        let config = AppConfig::load().unwrap_or_else(|e| {
            error!("Failed to load config: {}", e);
            AppConfig::default()
        });

        // Initialize i18n
        let i18n = I18n::new(config.language.clone());

        // Load games
        let games = Game::load_all().unwrap_or_default();

        info!("Loaded {} games", games.len());

        let main_window = MainWindow::new(config.clone(), i18n, games);

        // Initialize system tray if enabled
        let system_tray = if config.system_tray {
            let mut tray = SystemTray::new(TrayConfig {
                enabled: true,
                start_minimized: config.start_boot,
                close_to_tray: config.close_on_launch,
                show_notifications: true,
                is_mono: config.mono_icon,
                icon_path: None,
            });

            // Initialize the tray
            if let Err(e) = tray.init(main_window.i18n()) {
                error!("Failed to initialize system tray: {}", e);
                None
            } else {
                info!("System tray initialized");
                Some(tray)
            }
        } else {
            None
        };

        (
            Self {
                main_window,
                dialog: DialogState::None,
                pending_delete_index: None,
                system_tray,
                mouse_position: Point::ORIGIN,
            },
            Task::done(Message::Loaded),
        )
    }

    fn title(&self) -> String {
        self.main_window.title()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded => {
                info!("Application loaded");
                Task::none()
            }
            Message::ShowAddGameDialog => {
                let dialog = AddGameDialog::new(self.main_window.config(), self.main_window.i18n());
                self.dialog = DialogState::AddGame(Box::new(dialog));
                Task::none()
            }
            Message::ShowEditGameDialog(index) => {
                if let Some(game) = self.main_window.games().get(index).cloned() {
                    let dialog = AddGameDialog::edit(
                        game,
                        self.main_window.config(),
                        self.main_window.i18n(),
                    );
                    self.dialog = DialogState::AddGame(Box::new(dialog));
                }
                Task::none()
            }
            Message::ShowSettingsDialog => {
                let config = self.main_window.config().clone();
                let dialog = SettingsDialog::new(config);
                self.dialog = DialogState::Settings(Box::new(dialog));
                Task::none()
            }
            Message::ShowLogViewerDialog => {
                let dialog = LogViewerDialog::new();
                self.dialog = DialogState::LogViewer(Box::new(dialog));
                Task::none()
            }
            Message::ShowProtonManagerDialog => {
                let (dialog, task) = ProtonManagerDialog::new();
                self.dialog = DialogState::ProtonManager(Box::new(dialog));
                task.map(Message::ProtonManagerDialog)
            }
            Message::ShowConfirmationDialog(dialog) => {
                self.dialog = DialogState::Confirmation(dialog);
                Task::none()
            }
            Message::ConfirmationDialogClosed(confirmed) => {
                let remove_prefix = if let DialogState::Confirmation(dialog) = &self.dialog {
                    dialog.remove_prefix
                } else {
                    false
                };

                self.dialog = DialogState::None;

                // Handle confirmation result
                if let Some(index) = self.pending_delete_index {
                    if confirmed {
                        // User confirmed, proceed with deletion
                        return Task::done(Message::DeleteConfirmed(index, remove_prefix));
                    } else {
                        // User cancelled, clear pending state
                        self.pending_delete_index = None;
                    }
                }
                Task::none()
            }
            Message::ToggleRemovePrefix => {
                if let DialogState::Confirmation(dialog) = &mut self.dialog {
                    dialog.remove_prefix = !dialog.remove_prefix;
                }
                Task::none()
            }
            Message::DeleteClicked => {
                // Show confirmation dialog instead of deleting directly
                if let Some(index) = self.main_window.selected_game_index() {
                    if let Some(game) = self.main_window.games().get(index) {
                        self.pending_delete_index = Some(index);

                        let dialog = ConfirmationDialog::delete_confirmation(
                            game.title.clone(),
                            &game.prefix,
                            Message::ConfirmationDialogClosed(true),
                            Message::ConfirmationDialogClosed(false),
                        );

                        return Task::done(Message::ShowConfirmationDialog(Box::new(dialog)));
                    }
                }
                Task::none()
            }
            Message::DeleteConfirmed(index, remove_prefix) => {
                self.pending_delete_index = None;
                // Pass through to main_window for actual deletion
                self.main_window
                    .update(Message::DeleteConfirmed(index, remove_prefix))
            }
            Message::CloseDialog
            | Message::CloseAddGameDialog
            | Message::CloseSettingsDialog
            | Message::CloseLogViewerDialog
            | Message::CloseProtonManagerDialog
            | Message::CloseContextMenu => {
                self.dialog = DialogState::None;
                Task::none()
            }
            Message::NoOp => Task::none(),
            Message::MouseMoved(position) => {
                self.mouse_position = position;
                Task::none()
            }
            Message::GameRightClicked(index) => {
                if let Some(game) = self.main_window.games().get(index) {
                    let menu = ContextMenu::new(
                        index,
                        self.mouse_position,
                        game.title.clone(),
                        game.hidden,
                    );
                    self.dialog = DialogState::ContextMenu(Box::new(menu));
                }
                Task::none()
            }
            Message::ContextMenu(msg) => {
                if let DialogState::ContextMenu(menu) = &self.dialog {
                    let game_index = menu.game_index;
                    self.dialog = DialogState::None;

                    match msg {
                        ContextMenuMessage::Play => {
                            return Task::done(Message::GameDoubleClicked(game_index));
                        }
                        ContextMenuMessage::Edit => {
                            return Task::done(Message::ShowEditGameDialog(game_index));
                        }
                        ContextMenuMessage::Delete => {
                            // Ensure game is selected before triggering delete
                            let _ = self
                                .main_window
                                .update(Message::GameSelected(Some(game_index)));
                            return Task::done(Message::DeleteClicked);
                        }
                        ContextMenuMessage::Duplicate => {
                            // Ensure game is selected before triggering duplicate
                            let _ = self
                                .main_window
                                .update(Message::GameSelected(Some(game_index)));
                            return Task::done(Message::DuplicateClicked);
                        }
                        ContextMenuMessage::ToggleHidden => {
                            // Ensure game is selected before triggering toggle hidden
                            let _ = self
                                .main_window
                                .update(Message::GameSelected(Some(game_index)));
                            return Task::done(Message::HideShowClicked);
                        }
                        ContextMenuMessage::OpenLocation => {
                            if let Some(game) = self.main_window.games().get(game_index) {
                                let path = std::path::Path::new(&game.path);
                                if let Some(parent) = path.parent() {
                                    let _ = open::that(parent);
                                }
                            }
                        }
                        ContextMenuMessage::OpenPrefix => {
                            if let Some(game) = self.main_window.games().get(game_index) {
                                let _ = open::that(&game.prefix);
                            }
                        }
                        ContextMenuMessage::ShowLogs => {
                            return Task::done(Message::ShowLogViewerDialog);
                        }
                    }
                }
                Task::none()
            }
            Message::AddGameDialog(msg) => {
                // Handle dialog messages
                let should_close = match &mut self.dialog {
                    DialogState::AddGame(dialog) => {
                        match &msg {
                            AddGameMessage::Confirm => {
                                if dialog.validate() {
                                    let mut game = dialog.get_game();
                                    let add_steam_shortcut = dialog.shortcut_steam();
                                    let add_desktop_shortcut = dialog.shortcut_desktop();
                                    let add_appmenu_shortcut = dialog.shortcut_appmenu();

                                    // Handle banner
                                    if let Some(banner_path) = &game.banner {
                                        if banner_path.exists() {
                                            let banners_dir = config::paths::Paths::banners_dir();
                                            let _ = std::fs::create_dir_all(&banners_dir);
                                            let extension = banner_path
                                                .extension()
                                                .and_then(|e| e.to_str())
                                                .unwrap_or("png");
                                            let target_path = banners_dir
                                                .join(format!("{}.{}", game.gameid, extension));

                                            if banner_path != &target_path {
                                                if let Err(e) =
                                                    std::fs::copy(banner_path, &target_path)
                                                {
                                                    error!("Failed to copy banner: {}", e);
                                                } else {
                                                    game.banner = Some(target_path);
                                                }
                                            }
                                        }
                                    }

                                    // Save the game
                                    if let Err(e) = game.save() {
                                        error!("Failed to save game: {}", e);
                                    } else {
                                        info!("Game saved: {}", game.title);

                                        // Extract icon from executable if not exists
                                        if !IconManager::icon_exists(&game.gameid) {
                                            if let Err(e) = IconManager::extract_from_exe(
                                                &game.path,
                                                &game.gameid,
                                            ) {
                                                warn!(
                                                    "Failed to extract icon for {}: {}",
                                                    game.title, e
                                                );
                                            } else {
                                                info!("Icon extracted for: {}", game.title);
                                            }
                                        }

                                        // Handle Steam shortcut
                                        if add_steam_shortcut {
                                            if let Ok(mut shortcuts) = SteamShortcuts::load() {
                                                if let Err(e) = shortcuts.add_or_update(&game) {
                                                    error!("Failed to add Steam shortcut: {}", e);
                                                } else if let Err(e) = shortcuts.save() {
                                                    error!("Failed to save Steam shortcuts: {}", e);
                                                } else {
                                                    info!(
                                                        "Steam shortcut added for: {}",
                                                        game.title
                                                    );
                                                }
                                            }
                                        }

                                        // Handle desktop shortcuts
                                        let shortcut_location = match (
                                            add_desktop_shortcut,
                                            add_appmenu_shortcut,
                                        ) {
                                            (true, true) => ShortcutLocation::Both,
                                            (true, false) => ShortcutLocation::Desktop,
                                            (false, true) => ShortcutLocation::Applications,
                                            (false, false) => {
                                                // Check if shortcuts exist and remove them
                                                if DesktopShortcutManager::exists(&game) {
                                                    if let Err(e) =
                                                        DesktopShortcutManager::remove(&game)
                                                    {
                                                        error!("Failed to remove desktop shortcuts: {}", e);
                                                    }
                                                }
                                                ShortcutLocation::Both // Dummy value, won't be used
                                            }
                                        };

                                        if add_desktop_shortcut || add_appmenu_shortcut {
                                            if let Err(e) = DesktopShortcutManager::create(
                                                &game,
                                                shortcut_location,
                                            ) {
                                                error!("Failed to create desktop shortcuts: {}", e);
                                            } else {
                                                info!(
                                                    "Desktop shortcuts created for: {}",
                                                    game.title
                                                );
                                            }
                                        }

                                        // Reload games list
                                        self.main_window.reload_games();
                                    }
                                    true
                                } else {
                                    false
                                }
                            }
                            AddGameMessage::Cancel => true,
                            AddGameMessage::WinetricksClicked => {
                                let prefix = dialog.get_game().prefix;
                                let runner = dialog.get_game().runner;
                                if let Err(e) = wine_tools::run_winetricks(&prefix, &runner) {
                                    error!("Failed to run Winetricks: {}", e);
                                }
                                false
                            }
                            AddGameMessage::WinecfgClicked => {
                                let game = dialog.get_game();
                                if let Err(e) = wine_tools::run_winecfg(
                                    &game.prefix,
                                    &game.runner,
                                    Some(&game.gameid),
                                ) {
                                    error!("Failed to run Winecfg: {}", e);
                                }
                                false
                            }
                            _ => {
                                return dialog.update(msg.clone()).map(Message::AddGameDialog);
                            }
                        }
                    }
                    DialogState::None => false,
                    DialogState::Settings(_) => false,
                    DialogState::Confirmation(_) => false,
                    DialogState::LogViewer(_) => false,
                    DialogState::ProtonManager(_) => false,
                    DialogState::ContextMenu(_) => false,
                };

                if should_close {
                    self.dialog = DialogState::None;
                }

                Task::none()
            }
            Message::SettingsDialog(msg) => {
                // Handle settings dialog messages
                let should_close = match &mut self.dialog {
                    DialogState::Settings(dialog) => {
                        match &msg {
                            SettingsMessage::Confirm => {
                                // Save the configuration
                                let config = dialog.get_config().clone();
                                if let Err(e) = config.save() {
                                    error!("Failed to save config: {}", e);
                                } else {
                                    info!("Settings saved");
                                    // Update main window config
                                    self.main_window.update_config(config);
                                }
                                dialog.needs_restart()
                            }
                            SettingsMessage::Cancel => true,
                            SettingsMessage::ShowLogsClicked => {
                                // Open the log viewer dialog
                                return Task::done(Message::ShowLogViewerDialog);
                            }
                            SettingsMessage::ProtonManagerClicked => {
                                // Open the proton manager dialog
                                return Task::done(Message::ShowProtonManagerDialog);
                            }
                            SettingsMessage::WinetricksClicked => {
                                let config = dialog.get_config();
                                if let Err(e) = wine_tools::run_winetricks(
                                    &config.default_prefix,
                                    &config.default_runner,
                                ) {
                                    error!("Failed to run Winetricks: {}", e);
                                }
                                false
                            }
                            SettingsMessage::WinecfgClicked => {
                                let config = dialog.get_config();
                                if let Err(e) = wine_tools::run_winecfg(
                                    &config.default_prefix,
                                    &config.default_runner,
                                    None,
                                ) {
                                    error!("Failed to run Winecfg: {}", e);
                                }
                                false
                            }
                            _ => {
                                return dialog.update(msg.clone()).map(Message::SettingsDialog);
                            }
                        }
                    }
                    DialogState::None => false,
                    DialogState::AddGame(_) => false,
                    DialogState::Confirmation(_) => false,
                    DialogState::LogViewer(_) => false,
                    DialogState::ProtonManager(_) => false,
                    DialogState::ContextMenu(_) => false,
                };

                if should_close {
                    self.dialog = DialogState::None;
                }

                Task::none()
            }
            Message::LogViewerDialog(msg) => {
                // Handle log viewer dialog messages
                let should_close = match &mut self.dialog {
                    DialogState::LogViewer(dialog) => match &msg {
                        LogViewerMessage::Close => true,
                        _ => {
                            dialog.update(msg);
                            false
                        }
                    },
                    DialogState::None => false,
                    DialogState::AddGame(_) => false,
                    DialogState::Settings(_) => false,
                    DialogState::Confirmation(_) => false,
                    DialogState::ProtonManager(_) => false,
                    DialogState::ContextMenu(_) => false,
                };

                if should_close {
                    self.dialog = DialogState::None;
                }

                Task::none()
            }
            Message::ProtonManagerDialog(msg) => {
                // Handle proton manager dialog messages
                let should_close = match &mut self.dialog {
                    DialogState::ProtonManager(dialog) => match &msg {
                        ProtonManagerMessage::Close => true,
                        _ => {
                            return dialog.update(msg.clone()).map(Message::ProtonManagerDialog);
                        }
                    },
                    DialogState::None => false,
                    DialogState::AddGame(_) => false,
                    DialogState::Settings(_) => false,
                    DialogState::Confirmation(_) => false,
                    DialogState::LogViewer(_) => false,
                    DialogState::ContextMenu(_) => false,
                };

                if should_close {
                    self.dialog = DialogState::None;
                }

                Task::none()
            }
            Message::HideShowClicked => {
                // Toggle hidden state for selected game
                if let Some(index) = self.main_window.selected_game_index() {
                    if let Some(game) = self.main_window.games().get(index).cloned() {
                        // Toggle the hidden state
                        let new_hidden_state = !game.hidden;

                        // Update the game directly
                        if let Err(e) = game.update_hidden(new_hidden_state) {
                            error!("Failed to update game hidden state: {}", e);
                        } else {
                            info!(
                                "Game '{}' is now: {}",
                                game.title,
                                if new_hidden_state {
                                    "hidden"
                                } else {
                                    "visible"
                                }
                            );
                            // Reload games to reflect the change
                            self.main_window.reload_games();
                        }
                    }
                }
                Task::none()
            }
            Message::DuplicateClicked => {
                // Duplicate the selected game
                if let Some(index) = self.main_window.selected_game_index() {
                    if let Some(game) = self.main_window.games().get(index).cloned() {
                        info!("Duplicating game: {}", game.title);

                        // Create a duplicated game with new ID and (Copy) suffix
                        let new_game = game.duplicate();

                        // Save the duplicated game
                        if let Err(e) = new_game.save() {
                            error!("Failed to save duplicated game: {}", e);
                        } else {
                            info!("Game duplicated: {} -> {}", game.title, new_game.title);

                            // Copy icon from original game to new game
                            use icons::IconManager;
                            if IconManager::icon_exists(&game.gameid) {
                                let original_icon = IconManager::get_icon_path(&game.gameid);
                                if let Err(e) =
                                    IconManager::copy_custom_icon(&original_icon, &new_game.gameid)
                                {
                                    warn!("Failed to copy icon for duplicated game: {}", e);
                                } else {
                                    info!("Icon copied for: {}", new_game.title);
                                }
                            }

                            // Reload games to reflect the change
                            self.main_window.reload_games();

                            // Open the edit dialog for the duplicated game
                            return Task::done(Message::ShowEditGameDialog(
                                self.main_window
                                    .games()
                                    .iter()
                                    .position(|g| g.gameid == new_game.gameid)
                                    .unwrap_or_default(),
                            ));
                        }
                    }
                }
                Task::none()
            }
            Message::KillAllProcesses => {
                info!("Killing all game processes");
                self.main_window.launch_controller().terminate_all();
                Task::none()
            }
            Message::TrayEvent(event) => {
                // Handle system tray events
                match event {
                    TrayEvent::Show => {
                        // Show window
                        if let Some(tray) = &mut self.system_tray {
                            tray.set_window_visible(true);
                        }
                        info!("Window shown from tray");
                    }
                    TrayEvent::Hide => {
                        // Hide window
                        if let Some(tray) = &mut self.system_tray {
                            tray.set_window_visible(false);
                        }
                        info!("Window hidden to tray");
                    }
                    TrayEvent::Quit => {
                        // Quit application
                        info!("Quit requested from tray");
                        std::process::exit(0);
                    }
                    TrayEvent::TrayIconClicked | TrayEvent::TrayIconDoubleClicked => {
                        // Toggle window visibility
                        // For now, just show the window
                        if let Some(tray) = &mut self.system_tray {
                            tray.set_window_visible(true);
                        }
                    }
                }
                Task::none()
            }
            _ => self.main_window.update(message),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let main_content = mouse_area(self.main_window.view()).on_move(Message::MouseMoved);

        if let DialogState::None = self.dialog {
            return main_content.into();
        }

        if let DialogState::ContextMenu(menu) = &self.dialog {
            let menu_content = menu.view(self.main_window.i18n()).map(Message::ContextMenu);

            let overlay = mouse_area(
                container(menu_content)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(Padding {
                        top: menu.position.y,
                        left: menu.position.x,
                        ..Padding::ZERO
                    }),
            )
            .on_press(Message::CloseContextMenu);

            return stack![main_content, overlay].into();
        }

        let dialog_content: Element<'_, Message> = match &self.dialog {
            DialogState::None => unreachable!(),
            DialogState::ContextMenu(_) => unreachable!(),
            DialogState::AddGame(dialog) => container(
                dialog
                    .view(self.main_window.i18n())
                    .map(Message::AddGameDialog),
            )
            .width(Length::Fixed(600.0))
            .max_height(700.0)
            .padding(20)
            .into(),

            DialogState::Settings(dialog) => container(
                dialog
                    .view(self.main_window.i18n())
                    .map(Message::SettingsDialog),
            )
            .width(Length::Fixed(800.0))
            .max_height(700.0)
            .padding(20)
            .into(),

            DialogState::LogViewer(dialog) => container(
                dialog
                    .view(self.main_window.i18n())
                    .map(Message::LogViewerDialog),
            )
            .width(Length::Fixed(900.0))
            .max_height(700.0)
            .padding(20)
            .style(container::bordered_box)
            .into(),

            DialogState::ProtonManager(dialog) => container(
                dialog
                    .view(self.main_window.i18n())
                    .map(Message::ProtonManagerDialog),
            )
            .width(Length::Fixed(800.0))
            .max_height(700.0)
            .padding(20)
            .style(container::bordered_box)
            .into(),

            DialogState::Confirmation(dialog) => dialog.view(self.main_window.i18n()),
        };

        let backdrop = mouse_area(container(Space::new(Length::Fill, Length::Fill)).style(
            |_theme| container::Style {
                background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.8).into()),
                ..Default::default()
            },
        ))
        .on_press(Message::CloseDialog);

        let modal = stack![
            backdrop,
            container(mouse_area(dialog_content).on_press(Message::NoOp))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
        ];

        stack![main_content, modal].into()
    }
}

fn main() -> iced::Result {
    #[cfg(target_os = "linux")]
    {
        if let Err(e) = gtk::init() {
            eprintln!("Failed to initialize GTK: {}", e);
        }
    }

    // Load application icon
    let app_icon = config::paths::Paths::get_app_icon(false)
        .and_then(|path| iced::window::icon::from_file(path).ok());

    iced::application(
        "Faugus Launcher",
        FaugusLauncher::update,
        FaugusLauncher::view,
    )
    .subscription(FaugusLauncher::subscription)
    .window(window::Settings {
        size: Size::new(1200.0, 800.0),
        icon: app_icon,
        ..Default::default()
    })
    .run_with(FaugusLauncher::new)
}
