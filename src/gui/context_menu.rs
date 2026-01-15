use crate::locale::i18n::I18n;
use iced::widget::{button, column, container, text};
use iced::{Element, Length, Point};

#[derive(Debug, Clone)]
pub enum ContextMenuMessage {
    OpenLocation,
    OpenPrefix,
    ShowLogs,
}

#[derive(Debug, Clone)]
pub struct ContextMenu {
    pub game_index: usize,
    pub position: Point,
}

impl ContextMenu {
    pub fn new(game_index: usize, position: Point) -> Self {
        Self {
            game_index,
            position,
        }
    }

    pub fn view(&self, i18n: &I18n) -> Element<'_, ContextMenuMessage> {
        let content = column![
            button(text(i18n.t("Open game location")))
                .on_press(ContextMenuMessage::OpenLocation)
                .width(Length::Fill)
                .padding(8)
                .style(button::secondary),
            button(text(i18n.t("Open prefix location")))
                .on_press(ContextMenuMessage::OpenPrefix)
                .width(Length::Fill)
                .padding(8)
                .style(button::secondary),
            button(text(i18n.t("Show logs")))
                .on_press(ContextMenuMessage::ShowLogs)
                .width(Length::Fill)
                .padding(8)
                .style(button::secondary),
        ]
        .spacing(2);

        container(content)
            .width(Length::Fixed(220.0))
            .padding(4)
            .style(container::bordered_box)
            .into()
    }
}
