// Confirmation dialog
// Reusable dialog for confirming critical actions

use iced::widget::{button, checkbox, column, container, row, text, Space};
use iced::{Alignment, Element, Length, Padding};

use crate::locale::I18n;
use crate::Message;

/// Confirmation dialog state
#[derive(Debug, Clone)]
pub struct ConfirmationDialog {
    /// Dialog title
    title: String,
    /// Dialog message
    message: String,
    /// Callback when confirmed
    on_confirm: Box<Message>,
    /// Callback when cancelled
    on_cancel: Box<Message>,
    /// Whether to remove the prefix folder
    pub remove_prefix: bool,
    /// Whether to show the remove prefix checkbox
    pub show_remove_prefix: bool,
}

impl ConfirmationDialog {
    /// Create a new confirmation dialog
    pub fn new(
        title: String,
        message: String,
        on_confirm: Message,
        on_cancel: Message,
        show_remove_prefix: bool,
    ) -> Self {
        Self {
            title,
            message,
            on_confirm: Box::new(on_confirm),
            on_cancel: Box::new(on_cancel),
            remove_prefix: false,
            show_remove_prefix,
        }
    }

    /// Create a delete confirmation dialog
    pub fn delete_confirmation(
        game_title: String,
        prefix: &std::path::Path,
        on_confirm: Message,
        on_cancel: Message,
    ) -> Self {
        let message = format!(
            "{}: {}\n{}",
            crate::locale::i18n::I18n::default().t("Game"),
            game_title,
            crate::locale::i18n::I18n::default().t("This action cannot be undone.")
        );

        // Only show remove prefix if it's not the default one
        let default_prefix = crate::config::paths::Paths::default_prefix();
        let show_remove_prefix =
            prefix != &default_prefix && prefix.to_string_lossy() != "default" && prefix.exists();

        Self::new(
            crate::locale::i18n::I18n::default().t("Confirm"),
            message,
            on_confirm,
            on_cancel,
            show_remove_prefix,
        )
    }

    /// Create a custom confirmation dialog
    pub fn custom(
        title: impl Into<String>,
        message: impl Into<String>,
        on_confirm: Message,
        on_cancel: Message,
    ) -> Self {
        Self::new(title.into(), message.into(), on_confirm, on_cancel, false)
    }

    /// View the confirmation dialog
    pub fn view(&self, i18n: &I18n) -> Element<'_, Message> {
        let mut content = column![
            // Title
            container(text(&self.title).size(20))
                .padding(20)
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center),
            // Message
            container(text(&self.message).size(14))
                .padding(20)
                .width(Length::Fill),
        ]
        .spacing(10);

        // Add checkbox if needed
        if self.show_remove_prefix {
            content = content.push(
                container(
                    checkbox(i18n.t("Remove prefix folder"), self.remove_prefix)
                        .on_toggle(|_| Message::ToggleRemovePrefix)
                        .size(16),
                )
                .padding(Padding {
                    left: 20.0,
                    right: 20.0,
                    ..Padding::ZERO
                }),
            );
        }

        // Buttons
        content = content.push(
            row![
                Space::new(Length::Fill, Length::Shrink),
                button(text(i18n.t("No")).size(14))
                    .on_press((*self.on_cancel).clone())
                    .padding(10)
                    .width(100),
                button(text(i18n.t("Yes")).size(14))
                    .on_press((*self.on_confirm).clone())
                    .padding(10)
                    .width(100),
            ]
            .spacing(10)
            .padding(20)
            .align_y(Alignment::Center),
        );

        // Dialog container
        container(content)
            .width(400)
            .max_width(400)
            .padding(20)
            .style(container::bordered_box)
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confirmation_dialog_creation() {
        let dialog = ConfirmationDialog::new(
            "Test Title".to_string(),
            "Test Message".to_string(),
            Message::CloseAddGameDialog,
            Message::CloseSettingsDialog,
            false,
        );

        assert_eq!(dialog.title, "Test Title");
        assert_eq!(dialog.message, "Test Message");
    }
}
