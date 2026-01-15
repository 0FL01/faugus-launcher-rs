// GUI module
// Main graphical user interface using Iced

pub mod add_game_dialog;
pub mod confirmation_dialog;
pub mod context_menu;
pub mod file_picker;
pub mod log_viewer_dialog;
pub mod main_window;
pub mod proton_manager_dialog;
pub mod settings_dialog;

use add_game_dialog::AddGameDialog;
use confirmation_dialog::ConfirmationDialog;
use context_menu::ContextMenu;
use log_viewer_dialog::LogViewerDialog;
use proton_manager_dialog::ProtonManagerDialog;
use settings_dialog::SettingsDialog;

/// Dialog state for the application
#[derive(Debug, Clone)]
pub enum DialogState {
    None,
    AddGame(Box<AddGameDialog>),
    Settings(Box<SettingsDialog>),
    Confirmation(Box<ConfirmationDialog>),
    LogViewer(Box<LogViewerDialog>),
    ProtonManager(Box<ProtonManagerDialog>),
    ContextMenu(Box<ContextMenu>),
}
