use iced::{alignment, Length, Renderer, Theme};
use iced::widget::{button, Button, text};
use crate::Message;
use crate::styles::{BlueButtonStyleSheet, GreenButtonStyleSheet, RedButtonStyleSheet};

pub fn start_button() -> Button<'static, Message, Theme, Renderer> {
    button(
        text("Start")
            .size(34)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
    ).on_press(Message::Start)
    .style(BlueButtonStyleSheet::new())
    .width(Length::Fixed(160.0))
    .height(Length::Fixed(80.0))
}

pub fn check_button() -> Button<'static, Message, Theme, Renderer> {
    button(
        text("Check")
            .size(28)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
    ).style(BlueButtonStyleSheet::new())
    .width(Length::Fixed(100.0))
    .height(Length::Fixed(60.0))
}

pub fn next_button() -> Button<'static, Message, Theme, Renderer> {
    button(
        text("Next")
            .size(28)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
    ).on_press(Message::Next)
    .style(BlueButtonStyleSheet::new())
    .width(Length::Fixed(100.0))
    .height(Length::Fixed(60.0))
}

pub fn finish_button() -> Button<'static, Message, Theme, Renderer> {
    button(
        text("Finish")
            .size(28)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
    ).on_press(Message::Finish)
    .style(GreenButtonStyleSheet::new())
    .width(Length::Fixed(100.0))
    .height(Length::Fixed(60.0))
}

pub fn restart_button() -> Button<'static, Message, Theme, Renderer> {
    button(
        text("Restart")
            .size(28)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
    ).on_press(Message::Restart)
    .style(BlueButtonStyleSheet::new())
    .width(Length::Fixed(120.0))
    .height(Length::Fixed(60.0))
}

pub fn quit_button() -> Button<'static, Message, Theme, Renderer> {
    button(
        text("Quit")
            .size(29)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
    ).on_press(Message::Quit)
    .style(RedButtonStyleSheet::new())
    .width(Length::Fixed(100.0))
    .height(Length::Fixed(60.0))
}