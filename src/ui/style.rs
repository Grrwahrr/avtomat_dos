use iced::{button, Background, Color, Vector};

pub enum Button {
    // Primary,
    Secondary,
    Success,
    Danger,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                // Button::Primary => Color::from_rgb(0.0, 0.5, 1.0),
                Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                Button::Success => Color::from_rgb(0.2, 0.7, 0.3),
                Button::Danger => Color::from_rgb(0.8, 0.2, 0.3),
            })),
            border_radius: 5.0,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active()
        }
    }
}
