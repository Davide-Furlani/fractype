use iced::{Border, Color};
use iced::border::Radius;
use iced::widget::button;

pub struct RedButtonStyleSheet;
impl RedButtonStyleSheet {
    pub fn new() -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(Self))
    }
}
impl button::StyleSheet for RedButtonStyleSheet {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.palette();
        let background = iced::Background::Color(Color::from_rgb8(248, 206, 204));
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(background),
            text_color: palette.text,
            border: Border{
                color: Color::from_rgb8(184, 84, 80),
                width: 2.0,
                radius: Radius::from(0),
            },
            shadow: Default::default(),
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.palette();
        let background = iced::Background::Color(Color::from_rgb8(240, 185, 185));
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(background),
            text_color: palette.text,
            border: Border{
                color: Color::from_rgb8(184, 84, 80),
                width: 2.0,
                radius: Radius::from(0),
            },
            shadow: Default::default(),
        }
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.palette();
        let background = iced::Background::Color(Color::from_rgb8(235, 160, 160));
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(background),
            text_color: palette.text,
            border: Border{
                color: Color::from_rgb8(184, 84, 80),
                width: 2.0,
                radius: Radius::from(0),
            },
            shadow: Default::default(),
        }
    }
}

pub struct BlueButtonStyleSheet;
impl BlueButtonStyleSheet {
    pub fn new() -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(Self))
    }
}
impl button::StyleSheet for BlueButtonStyleSheet {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.palette();
        let background = iced::Background::Color(Color::from_rgb8(177,221,240));
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(background),
            text_color: palette.text,
            border: Border{
                color: Color::from_rgb8(16,15,158),
                width: 2.0,
                radius: Radius::from(0),
            },
            shadow: Default::default(),
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.palette();
        let background = iced::Background::Color(Color::from_rgb8(175,205,230));
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(background),
            text_color: palette.text,
            border: Border{
                color: Color::from_rgb8(16,15,158),
                width: 2.0,
                radius: Radius::from(0),
            },
            shadow: Default::default(),
        }
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.palette();
        let background = iced::Background::Color(Color::from_rgb8(150,185,225));
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(background),
            text_color: palette.text,
            border: Border{
                color: Color::from_rgb8(16,15,158),
                width: 2.0,
                radius: Radius::from(0),
            },
            shadow: Default::default(),
        }
    }
    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        let background = iced::Background::Color(Color::from_rgb8(127,127,127));
        button::Appearance {
            background: Some(background),
            border: Border{
                color: Color::from_rgb8(75, 75, 75),
                width: 2.0,
                radius: Radius::from(0),
            },
            text_color: Color::from_rgb8(100,100,100),
            ..Default::default()
        }
    }
}

pub struct GreenButtonStyleSheet;
impl GreenButtonStyleSheet {
    pub fn new() -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(Self))
    }
}
impl button::StyleSheet for GreenButtonStyleSheet {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.palette();
        let background = iced::Background::Color(Color::from_rgb8(188,232,191));
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(background),
            text_color: palette.text,
            border: Border{
                color: Color::from_rgb8(130,179,102),
                width: 2.0,
                radius: Radius::from(0),
            },
            shadow: Default::default(),
        }
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.palette();
        let background = iced::Background::Color(Color::from_rgb8(172,225,175));
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(background),
            text_color: palette.text,
            border: Border{
                color: Color::from_rgb8(130,179,102),
                width: 2.0,
                radius: Radius::from(0),
            },
            shadow: Default::default(),
        }
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.palette();
        let background = iced::Background::Color(Color::from_rgb8(155,210,155));
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(background),
            text_color: palette.text,
            border: Border{
                color: Color::from_rgb8(130,179,102),
                width: 2.0,
                radius: Radius::from(0),
            },
            shadow: Default::default(),
        }
    }
}

pub struct LineFakeButton;
impl LineFakeButton {
    pub fn new() -> iced::theme::Button {
        iced::theme::Button::Custom(Box::new(Self))
    }
}
impl button::StyleSheet for LineFakeButton {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.palette();
        let background = iced::Background::Color(Color::BLACK);
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(background),
            text_color: palette.text,
            border: Border{
                color: Color::BLACK,
                width: 0.0,
                radius: Radius::from(0),
            },
            shadow: Default::default(),
        }
    }
    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.palette();
        let background = iced::Background::Color(Color::BLACK);
        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(background),
            text_color: palette.text,
            border: Border{
                color: Color::BLACK,
                width: 0.0,
                radius: Radius::from(0),
            },
            shadow: Default::default(),
        }
    }
}