use crate::locale::i18n::I18n;
use iced::widget::{button, column, container, horizontal_rule, text};
use iced::{Element, Length, Point};

#[derive(Debug, Clone)]
pub enum ContextMenuMessage {
    Play,
    Edit,
    Delete,
    Duplicate,
    ToggleHidden,
    OpenLocation,
    OpenPrefix,
    ShowLogs,
}

#[derive(Debug, Clone)]
pub struct ContextMenu {
    pub game_index: usize,
    pub position: Point,
    pub game_title: String,
    pub is_hidden: bool,
}

impl ContextMenu {
    pub fn new(game_index: usize, position: Point, game_title: String, is_hidden: bool) -> Self {
        Self {
            game_index,
            position,
            game_title,
            is_hidden,
        }
    }

    pub fn view(&self, i18n: &I18n) -> Element<'_, ContextMenuMessage> {
        let hide_show_label = if self.is_hidden {
            i18n.t("Show")
        } else {
            i18n.t("Hide")
        };

        let content = column![
            container(column![text(&self.game_title).size(14),].spacing(2)).padding(8),
            horizontal_rule(1),
            button(text(i18n.t("Play")))
                .on_press(ContextMenuMessage::Play)
                .width(Length::Fill)
                .padding(8)
                .style(button::secondary),
            button(text(i18n.t("Edit")))
                .on_press(ContextMenuMessage::Edit)
                .width(Length::Fill)
                .padding(8)
                .style(button::secondary),
            button(text(i18n.t("Delete")))
                .on_press(ContextMenuMessage::Delete)
                .width(Length::Fill)
                .padding(8)
                .style(button::secondary),
            button(text(i18n.t("Duplicate")))
                .on_press(ContextMenuMessage::Duplicate)
                .width(Length::Fill)
                .padding(8)
                .style(button::secondary),
            button(text(hide_show_label))
                .on_press(ContextMenuMessage::ToggleHidden)
                .width(Length::Fill)
                .padding(8)
                .style(button::secondary),
            horizontal_rule(1),
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
