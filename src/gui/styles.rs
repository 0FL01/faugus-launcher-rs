use iced::widget::{button, container};
use iced::{Border, Color, Shadow, Vector};

pub mod colors {
    use iced::Color;

    pub const ACCENT: Color = Color::from_rgb(0.0, 0.95, 1.0); // Electric Blue/Cyan
    pub const ACCENT_HOVER: Color = Color::from_rgb(0.4, 0.98, 1.0);
    pub const SURFACE: Color = Color::from_rgba(0.06, 0.08, 0.14, 0.6); // Dark Glass
    pub const SURFACE_HOVER: Color = Color::from_rgba(0.1, 0.15, 0.25, 0.7);
    pub const BORDER: Color = Color::from_rgba(0.3, 0.5, 0.7, 0.3);
    pub const MODAL_SURFACE: Color = Color::from_rgb(0.06, 0.08, 0.14); // Opaque Dark
    pub const TEXT_PRIMARY: Color = Color::WHITE;
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.7, 0.7, 0.7);
}

pub struct DeepSpace;

impl DeepSpace {
    pub fn container(_theme: &iced::Theme) -> container::Style {
        container::Style {
            text_color: Some(colors::TEXT_PRIMARY),
            background: Some(colors::SURFACE.into()),
            border: Border {
                color: colors::BORDER,
                width: 1.0,
                radius: 16.0.into(),
            },
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::new(0.0, 4.0),
                blur_radius: 10.0,
            },
        }
    }

    pub fn modal_container(_theme: &iced::Theme) -> container::Style {
        container::Style {
            text_color: Some(colors::TEXT_PRIMARY),
            background: Some(colors::MODAL_SURFACE.into()),
            border: Border {
                color: colors::ACCENT,
                width: 1.0,
                radius: 12.0.into(),
            },
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::new(0.0, 4.0),
                blur_radius: 20.0,
            },
        }
    }

    pub fn scrollable(
        _theme: &iced::Theme,
        _status: iced::widget::scrollable::Status,
    ) -> iced::widget::scrollable::Style {
        let rail = iced::widget::scrollable::Rail {
            background: Some(colors::SURFACE.into()),
            border: Border {
                color: colors::BORDER,
                width: 0.0,
                radius: 4.0.into(),
            },
            scroller: iced::widget::scrollable::Scroller {
                color: colors::ACCENT,
                border: Border {
                    color: colors::ACCENT,
                    width: 0.0,
                    radius: 4.0.into(),
                },
            },
        };

        iced::widget::scrollable::Style {
            container: container::Style::default(),
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: None,
        }
    }

    pub fn transparent_container(_theme: &iced::Theme) -> container::Style {
        container::Style::default()
    }

    pub fn button(_theme: &iced::Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                background: Some(colors::SURFACE.into()),
                text_color: colors::TEXT_PRIMARY,
                border: Border {
                    color: colors::BORDER,
                    width: 1.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow::default(),
            },
            button::Status::Hovered => button::Style {
                background: Some(colors::SURFACE_HOVER.into()),
                text_color: colors::ACCENT,
                border: Border {
                    color: colors::ACCENT,
                    width: 1.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow {
                    color: colors::ACCENT,
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 8.0,
                },
            },
            button::Status::Pressed => button::Style {
                background: Some(Color::from_rgba(0.0, 0.95, 1.0, 0.2).into()),
                text_color: Color::WHITE,
                border: Border {
                    color: colors::ACCENT,
                    width: 1.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow::default(),
            },
            button::Status::Disabled => button::Style {
                background: Some(Color::from_rgba(0.1, 0.1, 0.1, 0.3).into()),
                text_color: Color::from_rgb(0.5, 0.5, 0.5),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow::default(),
            },
        }
    }

    pub fn menu_button(_theme: &iced::Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                background: None,
                text_color: colors::TEXT_SECONDARY,
                border: Border::default(),
                shadow: Shadow::default(),
            },
            button::Status::Hovered => button::Style {
                background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.1).into()),
                text_color: colors::ACCENT,
                border: Border {
                    color: colors::ACCENT,
                    width: 0.0, // No border for menu items usually
                    radius: 8.0.into(),
                },
                shadow: Shadow::default(),
            },
            button::Status::Pressed => button::Style {
                background: Some(Color::from_rgba(0.0, 0.95, 1.0, 0.1).into()),
                text_color: colors::ACCENT,
                ..button::Style::default()
            },
            button::Status::Disabled => button::Style {
                text_color: colors::TEXT_SECONDARY,
                ..button::Style::default()
            },
        }
    }

    pub fn text_input(
        _theme: &iced::Theme,
        _status: iced::widget::text_input::Status,
    ) -> iced::widget::text_input::Style {
        iced::widget::text_input::Style {
            background: colors::SURFACE.into(),
            border: Border {
                color: colors::BORDER,
                width: 1.0,
                radius: 8.0.into(),
            },
            icon: colors::TEXT_SECONDARY,
            placeholder: colors::TEXT_SECONDARY,
            value: colors::TEXT_PRIMARY,
            selection: Color::from_rgba(0.0, 0.95, 1.0, 0.3),
        }
    }

    pub fn primary_button(theme: &iced::Theme, status: button::Status) -> button::Style {
        let base = Self::button(theme, status);
        match status {
            button::Status::Active => button::Style {
                background: Some(colors::ACCENT.into()),
                text_color: Color::BLACK,
                border: Border {
                    color: colors::ACCENT,
                    width: 1.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow::default(),
            },
            button::Status::Hovered => button::Style {
                background: Some(colors::ACCENT_HOVER.into()),
                text_color: Color::BLACK,
                border: Border {
                    color: colors::ACCENT_HOVER,
                    width: 1.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow {
                    color: colors::ACCENT,
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 12.0,
                },
            },
            button::Status::Pressed => button::Style {
                background: Some(Color::from_rgba(0.0, 0.8, 0.9, 1.0).into()),
                text_color: Color::BLACK,
                border: Border {
                    color: colors::ACCENT,
                    width: 1.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow::default(),
            },
            _ => base,
        }
    }

    pub fn checkbox(
        _theme: &iced::Theme,
        status: iced::widget::checkbox::Status,
    ) -> iced::widget::checkbox::Style {
        iced::widget::checkbox::Style {
            background: colors::SURFACE.into(),
            icon_color: colors::ACCENT,
            border: Border {
                color: match status {
                    iced::widget::checkbox::Status::Active { .. } => colors::BORDER,
                    iced::widget::checkbox::Status::Hovered { .. } => colors::ACCENT,
                    iced::widget::checkbox::Status::Disabled { .. } => {
                        Color::from_rgb(0.3, 0.3, 0.3)
                    }
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Some(colors::TEXT_PRIMARY),
        }
    }

    pub fn pick_list(
        _theme: &iced::Theme,
        status: iced::widget::pick_list::Status,
    ) -> iced::widget::pick_list::Style {
        let border_color = match status {
            iced::widget::pick_list::Status::Active => colors::BORDER,
            iced::widget::pick_list::Status::Hovered => colors::ACCENT,
            iced::widget::pick_list::Status::Opened => colors::ACCENT,
        };

        iced::widget::pick_list::Style {
            text_color: colors::TEXT_PRIMARY,
            placeholder_color: colors::TEXT_SECONDARY,
            background: colors::SURFACE.into(),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            handle_color: colors::ACCENT,
        }
    }

    pub fn main_window_container(_theme: &iced::Theme) -> container::Style {
        container::Style {
            background: Some(colors::MODAL_SURFACE.into()),
            text_color: Some(colors::TEXT_PRIMARY),
            ..Default::default()
        }
    }

    pub fn menu(_theme: &iced::Theme) -> iced::widget::overlay::menu::Style {
        iced::widget::overlay::menu::Style {
            text_color: colors::TEXT_PRIMARY,
            background: colors::MODAL_SURFACE.into(),
            border: Border {
                color: colors::ACCENT,
                width: 1.0,
                radius: 8.0.into(),
            },
            selected_text_color: Color::BLACK,
            selected_background: colors::ACCENT.into(),
        }
    }
}
