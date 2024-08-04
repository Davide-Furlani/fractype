#![windows_subsystem = "windows"]

mod styles;
mod buttons;

use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use iced::{Element, Renderer, Application, Settings, Theme, Command, window, executor, Size, Length, Padding, Alignment, Color};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, text, Column, Text, Container, Row, Button, radio};
use iced::window::close;
use rand::{distributions::{Distribution, Standard}, Rng};
use crate::buttons::{check_button, finish_button, next_button, quit_button, restart_button, start_button};
use crate::styles::{LineFakeButton};

#[derive(Debug, Clone)]
enum Message {
    Start,
    ReadInput(Choice),
    Check,
    Next,
    Finish,
    Restart,
    Quit
}
impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Start                  => {write!(f, "Start")},
            Message::ReadInput(s)   => {write!(f, "{}", s)}
            Message::Check                  => {write!(f, "Check")},
            Message::Next                   => {write!(f, "Next")},
            Message::Finish                 => {write!(f, "Finish")},
            Message::Restart                => {write!(f, "Restart")},
            Message::Quit                   => {write!(f, "Quit")},
        }
    }
}
impl Default for Message {
    fn default() -> Self {
        Message::Start
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mode {
    Start,
    Exercising,
    Result,
    FinalEvaluation
}
impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Start             => {write!(f, "Start")},
            Mode::Exercising        => {write!(f, "Exercising")},
            Mode::Result            => {write!(f, "Result")},
            Mode::FinalEvaluation   => {write!(f, "FinalEvaluation")},
        }
    }
}
impl Default for Mode {
    fn default() -> Self {
        Mode::Start
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Ftype {
    Proper,
    Improper,
    Apparent,
}
impl Display for Ftype {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Ftype::Proper   => {write!(f, "Proper")},
            Ftype::Improper => {write!(f, "Improper")},
            Ftype::Apparent => {write!(f, "Apparent")},
        }
    }
}
impl Default for Ftype {
    fn default() -> Self {
        Ftype::Proper
    }
}
impl Distribution<Ftype> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ftype {
        match rng.gen_range(0..=2) {
            0 => Ftype::Proper,
            1 => Ftype::Improper,
            _ => Ftype::Apparent,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Choice {
    Proper,
    Improper,
    Apparent,
    Unselected,
}
impl Display for Choice{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Choice::Proper      => {write!(f, "Proper")},
            Choice::Improper    => {write!(f, "Improper")},
            Choice::Apparent    => {write!(f, "Apparent")},
            Choice::Unselected  => {write!(f, "Unselected")},
        }
    }
}
impl Default for Choice{
    fn default() -> Self {
        Choice::Unselected
    }
}

#[derive(Debug, Copy, Clone)]
struct State {
    mode: Mode,
    error_made: bool,
    errors_count: u32,
    exercise_count: u32,
    num: u32,
    den: u32,
    frac_type: Ftype,
    choice: Choice,
}
impl Default for State {
    fn default() -> Self {
        State{
            mode: Mode::default(),
            error_made: false,
            errors_count: 0,
            exercise_count: 0,
            num: 0,
            den: 0,
            frac_type: Ftype::default(),
            choice: Choice::default(),
        }
    }
}
impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "state: {}\nerror made: {}\terror count: {}\t exercise count: {}\nnum:  {}\tden:  {}\nfraction type:  {}\tchoice:  {}\n",
               self.mode, self.error_made, self.errors_count, self.exercise_count, self.num, self.den, self.frac_type, self.choice)
    }
}

impl State {
    fn get_new_numbers (&mut self) {

        self.frac_type = rand::random();

        let (num, den) = match self.frac_type {
            Ftype::Proper => {
                let p = rand::thread_rng().gen_range(1..=119);
                (p, rand::thread_rng().gen_range(p+1..=120))
            }
            Ftype::Improper => {
                let mut count = 0;
                let mut found = false;
                let mut num: u32 = 0;
                let mut den: u32 = 0;
                while !found {
                    num = rand::thread_rng().gen_range(2..=120);
                    den = rand::thread_rng().gen_range(1..num);
                    if num % den != 0 {
                        found = true;
                    }
                    count += 1;
                }
                println!("{}", count);
                (num, den)
            }
            Ftype::Apparent => {
                let mut count = 0;
                let mut found = false;
                let mut num: u32 = 0;
                let mut den: u32 = 0;
                while !found {
                    num = rand::thread_rng().gen_range(1..=119);
                    let mut divisors: Vec<u32> = vec![];
                    divisors.push(1);
                    divisors.push(num);
                    for i in 2..=(num as f32).sqrt() as u32 {
                        if num % i == 0 { divisors.push(i); }
                    }
                    if divisors.len() > 2 {
                        den = num / divisors[rand::thread_rng().gen_range(0..divisors.len())];
                        found = true;
                    }
                    count += 1;
                }
                if num == 0 || den == 0 { panic!("num or den found but still 0\n num: {}\t den: {}", num, den) };
                println!("{}", count);
                (num, den)
            }
        };

        self.num = num;
        self.den = den;
    }
    fn start (&mut self) {
        self.get_new_numbers();
        self.mode = Mode::Exercising;
    }
    fn read_input(&mut self, choice: Choice) {
        self.choice = choice;
    }
    fn evaluate (&mut self) {
        self.exercise_count += 1;
        match self.choice {
            Choice::Proper => {
                if self.frac_type != Ftype::Proper {
                    self.error_made = true;
                    self.errors_count += 1;
                }
            }
            Choice::Improper => {
                if self.frac_type != Ftype::Improper {
                    self.error_made = true;
                    self.errors_count += 1;
                }
            }
            Choice::Apparent => {
                if self.frac_type != Ftype::Apparent {
                    self.error_made = true;
                    self.errors_count += 1;
                }
            }
            Choice::Unselected => {
                panic!("Selected unselectable choice: {}", self.choice);
            }
        }
        self.mode = Mode::Result;
    }
    fn next (&mut self) {
        self.get_new_numbers();
        self.choice = Choice::default();
        self.error_made = false;
        self.mode = Mode::Exercising;
    }
    fn finish (&mut self) {
        self.mode = Mode::FinalEvaluation;
    }
    fn restart (&mut self) {
        self.mode = Mode::Exercising;
        self.error_made = false;
        self.errors_count = 0;
        self.exercise_count = 0;
        self.choice = Choice::default();
        self.get_new_numbers();
    }
}

impl Application for State{
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }
    fn title(&self) -> String {
        String::from("FracType")
    }
    fn update(&mut self, message: Self::Message) -> Command<Message>{
        match message {
            Message::Start                  => {
                self.start();
                //println!("Start");
                Command::none()
            }
            Message::ReadInput(input_string)    => {
                self.read_input(input_string);
                //println!("{}",self.input_number);
                Command::none()
            }
            Message::Check                  => {
                self.evaluate();
                //println!("Check");
                Command::none()
            }
            Message::Next                   => {
                self.next();
                //println!("Next");
                Command::none()
            }
            Message::Finish                 => {
                self.finish();
                //println!("Finish");
                Command::none()
            }
            Message::Restart                =>  {
                self.restart();
                //println!("Restart");
                Command::none()
            }
            Message::Quit                   => {
                //println!("Quit");
                close(window::Id::MAIN)
            }
        }
    }
    fn view(&self) -> Element<'_, Self::Message> {

        let start_button = start_button();

        let check_button = check_button();
        let check_button = if self.choice != Choice::Unselected {check_button.on_press(Message::Check)} else {check_button};

        let next_button = next_button();

        let finish_button = finish_button();

        let restart_button = restart_button();

        let quit_button = quit_button();

        let start_title: Text<'_, Theme, Renderer> = text("Exercises on the types of fractions").size(32);
        let result_labels: Text<'_, Theme, Renderer> = text("Correct:\nErrors:\nAccuracy:").size(41);
        let result_numbers: Text<'_, Theme, Renderer> = text(format!("{}\n{}\n{}%",
                                                                    self.exercise_count-self.errors_count,
                                                                    self.errors_count,
                                                                    (self.exercise_count as f32 - self.errors_count as f32) / self.exercise_count as f32 * 100.0)).size(41);

        let text_proper: Text<'_, Theme, Renderer> =
            if self.choice == Choice::Proper {
                if self.error_made {
                    text("Proper").size(35).style(Color::from_rgb8(184, 84, 80))
                } else {
                    text("Proper").size(35).style(Color::from_rgb8(130, 179, 102))
                }

            }
            else if self.frac_type == Ftype::Proper {
                text("Proper").size(35).style(Color::from_rgb8(130, 179, 102))
            }
            else {
                text("Proper").size(35)
            };
        let text_improper: Text<'_, Theme, Renderer> =
            if self.choice == Choice::Improper {
                if self.error_made {
                    text("Improper").size(35).style(Color::from_rgb8(184, 84, 80))
                } else {
                    text("Improper").size(35).style(Color::from_rgb8(130, 179, 102))
                }

            }
            else if self.frac_type == Ftype::Improper {
                text("Improper").size(35).style(Color::from_rgb8(130, 179, 102))
            }
            else {
                text("Improper").size(35)
            };
        let text_apparent: Text<'_, Theme, Renderer> =
            if self.choice == Choice::Apparent {
                if self.error_made {
                    text("Apparent").size(35).style(Color::from_rgb8(184, 84, 80))
                } else {
                    text("Apparent").size(35).style(Color::from_rgb8(130, 179, 102))
                }

            }
            else if self.frac_type == Ftype::Apparent {
                text("Apparent").size(35).style(Color::from_rgb8(130, 179, 102))
            }
            else {
                text("Apparent").size(35)
            };


        let numerator = text(self.num.to_string()).size(46);

        let denominator = text(self.den.to_string()).size(46);
        

        let line_a: Button<'_, Message, Theme, Renderer> = button("")
            .style(LineFakeButton::new())
            .width(Length::Fixed(120.0))
            .height(Length::Fixed(2.0));

        let selected_choice = Some(self.choice);

        let radio_proper = radio("Proper", Choice::Proper, selected_choice, Message::ReadInput).text_size(30);
        let radio_improper = radio("Improper", Choice::Improper, selected_choice, Message::ReadInput).text_size(30);
        let radio_apparent = radio("Apparent", Choice::Apparent, selected_choice, Message::ReadInput).text_size(30);

        match self.mode{
            Mode::Start => {
                Column::new()
                    .width(Length::Fill)
                    .align_items(Alignment::Center)
                    .push(Container::new(start_title).padding(Padding{top: 60.0, right: 0.0, bottom: 0.0, left: 0.0}))
                    .push(Container::new(start_button).padding(Padding{top: 50.0, right: 0.0, bottom: 0.0, left: 0.0}))
                    .push(Container::new(quit_button).padding(Padding{top: 50.0, right: 0.0, bottom: 0.0, left: 0.0}))
                    .into()
            }
            Mode::Exercising => {
                Column::new()
                    .push(Container::new(Row::new()
                        .push(Container::new(Column::new()
                                .push(numerator)
                                .push(Container::new(line_a).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                .push(denominator)
                                .align_items(Alignment::Center)
                            ).height(Length::Fill)
                            .align_y(Vertical::Center)
                            .align_x(Horizontal::Right)
                        )
                        .push(Container::new(Column::new()
                                .push(radio_proper)
                                .push(Container::new(radio_improper).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                .push(radio_apparent)
                                .align_items(Alignment::Start)
                            ).height(Length::Fill)
                            .align_y(Vertical::Center)
                            .align_x(Horizontal::Center)
                            .padding(Padding{top: 0.0, right: 0.0, bottom: 0.0, left: 50.0})
                        ))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                    )
                    .push(Container::new(check_button)
                        .width(Length::Fill)
                        .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 450.0})
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_items(Alignment::Center)
                    .into()
            }
            Mode::Result => {
                Column::new()
                    .push(Container::new(Row::new()
                        .push(Container::new(Column::new()
                                .push(numerator)
                                .push(Container::new(line_a).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                .push(denominator)
                                .align_items(Alignment::Center)
                            ).height(Length::Fill)
                            .align_y(Vertical::Center)
                            .align_x(Horizontal::Right)
                        )
                        .push(Container::new(Column::new()
                                .push(text_proper)
                                .push(Container::new(text_improper).padding(Padding{top: 20.0, right: 0.0, bottom: 20.0, left: 0.0}))
                                .push(text_apparent)
                                .align_items(Alignment::Start)
                            ).height(Length::Fill)
                            .align_y(Vertical::Center)
                            .align_x(Horizontal::Center)
                            .padding(Padding{top: 0.0, right: 0.0, bottom: 0.0, left: 70.5})
                        ))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                    )
                    .push(Container::new(if self.exercise_count <20 {next_button} else {finish_button})
                        .width(Length::Fill)
                        .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 450.0})
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_items(Alignment::Center)
                    .into()
            }
            Mode::FinalEvaluation => {
                Column::new()
                    .push(
                        Container::new(
                            Column::new()
                                .push(Container::new(
                                    Row::new()
                                        .push(Container::new(result_labels).align_x(Horizontal::Left).padding(Padding{top: 0.0, right: 50.0, bottom: 0.0, left: 0.0}))
                                        .push(Container::new(result_numbers).align_x(Horizontal::Right))
                                ).padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 0.0}))
                        )
                        .align_y(Vertical::Center)
                        .align_x(Horizontal::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)
                    )
                    .push(
                        Container::new(Row::new()
                            .push(Container::new(quit_button)
                                .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 50.0})
                            )
                            .push(Container::new(restart_button)
                                .padding(Padding{top: 0.0, right: 0.0, bottom: 30.0, left: 280.0})
                            )
                        ).align_x(Horizontal::Left)
                        .width(Length::Fill)
                    )
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .into()
            }
        }

    }
    fn theme(&self) -> Self::Theme {
        Theme::Light
    }
}


static ICON: &[u8] = include_bytes!("../res/icon/fractype_512.png");
const ICON_HEIGHT: u32 = 512;
const ICON_WIDTH: u32 = 512;

fn main() -> iced::Result {

    let image = image::load_from_memory(ICON).unwrap();
    let icon = window::icon::from_rgba(image.as_bytes().to_vec(), ICON_HEIGHT, ICON_WIDTH).unwrap();

    let settings = Settings{
        id: None,
        window: window::settings::Settings{
            size: Size { width: 600.0, height: 400.0 },
            position: window::Position::Centered,
            resizable: false,
            icon: Some(icon),
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    };

    State::run(settings)
}