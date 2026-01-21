// Log Viewer Dialog
// Simple dialog for viewing and managing application logs

use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Element, Length};
use std::fs;
use std::path::PathBuf;
use tracing::{error, info, warn};

use crate::config::paths::Paths;
use crate::locale::I18n;

/// Messages for the Log Viewer dialog
#[derive(Debug, Clone)]
pub enum LogViewerMessage {
    /// Refresh logs
    Refresh,
    /// Clear current log file
    ClearLogs,
    /// Open log directory in file manager
    OpenLogDirectory,
    /// Close dialog
    Close,
}

/// State for the Log Viewer dialog
#[derive(Debug, Clone)]
pub struct LogViewerDialog {
    /// Current log content
    log_content: String,
    /// Current log file path
    log_file_path: Option<PathBuf>,
}

impl LogViewerDialog {
    /// Create a new Log Viewer dialog
    pub fn new() -> Self {
        let mut dialog = Self {
            log_content: String::new(),
            log_file_path: None,
        };

        dialog.load_latest_log();
        dialog
    }

    /// Load the most recent log file
    fn load_latest_log(&mut self) {
        let logs_dir = Paths::logs_dir();

        // Create logs directory if it doesn't exist
        if !logs_dir.exists() {
            if let Err(e) = fs::create_dir_all(&logs_dir) {
                error!("Failed to create logs directory: {}", e);
            }
        }

        // Find the most recent log file
        if let Ok(entries) = fs::read_dir(&logs_dir) {
            let mut most_recent: Option<(PathBuf, std::time::SystemTime)> = None;

            for entry in entries.flatten() {
                let path = entry.path();
                if path
                    .extension()
                    .is_some_and(|ext| ext == "log" || ext == "txt")
                {
                    if let Ok(metadata) = fs::metadata(&path) {
                        if let Ok(modified) = metadata.modified() {
                            let is_more_recent =
                                most_recent.as_ref().is_none_or(|(_, m)| modified > *m);
                            if is_more_recent {
                                most_recent = Some((path, modified));
                            }
                        }
                    }
                }
            }

            if let Some((path, _)) = most_recent {
                self.log_file_path = Some(path.clone());
                self.log_content = fs::read_to_string(&path)
                    .unwrap_or_else(|e| format!("Failed to read log file: {}", e));
            } else {
                self.log_content =
                    "No log files found. Logs will appear here once the application has been run."
                        .to_string();
            }
        } else {
            self.log_content =
                "Logs directory not found. Enable logging in Settings to create log files."
                    .to_string();
        }

        // If log is empty, show a message
        if self.log_content.is_empty() && self.log_file_path.is_some() {
            self.log_content =
                "Log file is empty. Logs will appear here as the application runs.".to_string();
        }
    }

    /// Update the dialog state
    pub fn update(&mut self, message: LogViewerMessage) {
        match message {
            LogViewerMessage::Refresh => {
                self.load_latest_log();
                info!("Log viewer refreshed");
            }
            LogViewerMessage::ClearLogs => {
                if let Some(path) = &self.log_file_path {
                    if let Err(e) = fs::write(path, "") {
                        error!("Failed to clear log file: {}", e);
                    } else {
                        info!("Cleared log file: {:?}", path);
                        self.load_latest_log();
                    }
                }
            }
            LogViewerMessage::OpenLogDirectory => {
                let logs_dir = Paths::logs_dir();
                info!("Opening log directory: {:?}", logs_dir);

                // Try to open with xdg-open
                if std::process::Command::new("xdg-open")
                    .arg(&logs_dir)
                    .spawn()
                    .is_ok()
                {
                    info!("Opened log directory with xdg-open");
                } else {
                    warn!("Failed to open log directory");
                }
            }
            LogViewerMessage::Close => {
                // Handled by parent
            }
        }
    }

    /// View the dialog
    pub fn view(&self, i18n: &I18n) -> Element<'_, LogViewerMessage> {
        // Header with title
        let header = container(text(i18n.t("Application Logs")).size(20))
            .padding(10)
            .width(Length::Fill);

        // File info
        let file_info = if let Some(path) = &self.log_file_path {
            text(format!(
                "File: {}",
                path.file_name().unwrap_or_default().to_string_lossy()
            ))
            .size(12)
        } else {
            text("No log file").size(12)
        };

        // Log content display
        let log_display = self.view_log_display();

        // Buttons
        let buttons = self.view_buttons(i18n);

        column![
            header,
            Space::with_height(Length::Fixed(10.0)),
            file_info,
            Space::with_height(Length::Fixed(10.0)),
            log_display,
            Space::with_height(Length::Fixed(10.0)),
            buttons,
        ]
        .spacing(5)
        .padding(20)
        .into()
    }

    /// View log display section
    fn view_log_display(&self) -> Element<'_, LogViewerMessage> {
        let log_text = text(&self.log_content).size(11).width(Length::Fill);

        let scrollable_content = scrollable(log_text)
            .width(Length::Fill)
            .height(Length::Fill);

        container(scrollable_content)
            .width(Length::Fill)
            .height(Length::Fixed(500.0))
            .padding(10)
            .into()
    }

    /// View buttons section
    fn view_buttons(&self, i18n: &I18n) -> Element<'_, LogViewerMessage> {
        row![
            button(text(i18n.t("Refresh")).size(14)).on_press(LogViewerMessage::Refresh),
            Space::with_width(Length::Fixed(10.0)),
            button(text(i18n.t("Clear Logs")).size(14)).on_press(LogViewerMessage::ClearLogs),
            Space::with_width(Length::Fixed(10.0)),
            button(text(i18n.t("Open Directory")).size(14))
                .on_press(LogViewerMessage::OpenLogDirectory),
            Space::with_width(Length::Fill),
            button(text(i18n.t("Close")).size(14)).on_press(LogViewerMessage::Close),
        ]
        .spacing(5)
        .align_y(iced::alignment::Vertical::Center)
        .into()
    }
}
