use iced::widget::{button, container, text};
use iced::{Border, Color, Shadow, Vector};

pub mod colors {
    use iced::Color;

    pub const ACCENT: Color = Color::from_rgb(0.0, 0.95, 1.0); // Electric Blue/Cyan
    pub const ACCENT_HOVER: Color = Color::from_rgb(0.4, 0.98, 1.0);
    pub const SURFACE: Color = Color::from_rgba(0.06, 0.08, 0.14, 0.6); // Dark Glass
    pub const SURFACE_HOVER: Color = Color::from_rgba(0.1, 0.15, 0.25, 0.7);
    pub const BORDER: Color = Color::from_rgba(0.3, 0.5, 0.7, 0.3);
    pub const TEXT_PRIMARY: Color = Color::WHITE;
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.7, 0.7, 0.7);
}

pub struct DeepSpace;

impl DeepSpace {
    pub fn container(theme: &iced::Theme) -> container::Style {
        container::Style {
            text_color: Some(colors::TEXT_PRIMARY),
            background: Some(colors::SURFACE.into()),
            border: Border {
                color: colors::BORDER,
                width: 1.0,
                radius: 16.0.into(),
            },
            shadow: Shadow {
                color: Color::BLACK.into(),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 10.0,
            },
        }
    }

    pub fn transparent_container(theme: &iced::Theme) -> container::Style {
        container::Style::default()
    }

    pub fn button(theme: &iced::Theme, status: button::Status) -> button::Style {
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
                    color: colors::ACCENT.into(),
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
}
