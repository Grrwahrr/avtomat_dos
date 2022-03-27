use crate::crawler::Crawler;
use iced::{
    button, scrollable, slider, time, Application, Button, Clipboard, Column, Command, Container,
    Element, HorizontalAlignment, Length, Row, Scrollable, Slider, Space, Subscription, Text,
};
use std::time::Instant;

mod style;

/// Messages from the UI
#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    ButtonStart,
    ButtonScreenMain,
    ButtonScreenTargets,
    IntensityChanged(u8),
}

/// The different screens displayed by the app
enum Screen {
    Main,
    Targets,
}

pub struct UserInterface {
    screen_active: Screen,
    slider_intensity: slider::State,
    button_start: button::State,
    button_view_main: button::State,
    button_view_targets: button::State,
    scroll: scrollable::State,
    state: bool,
    intensity: u8,
    crawler: Crawler,
}

impl Application for UserInterface {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                screen_active: Screen::Main,
                slider_intensity: slider::State::new(),
                button_start: button::State::new(),
                button_view_main: button::State::new(),
                button_view_targets: button::State::new(),
                scroll: scrollable::State::new(),
                state: false,
                intensity: 50,
                crawler: Crawler::new(),
            },
            Command::none(),
        )
    }

    /// Title for the window
    fn title(&self) -> String {
        format!("AvtomatDoS {}", env!("CARGO_PKG_VERSION"))
    }

    fn update(&mut self, event: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match event {
            Message::Tick(_) => {
                // This will cause the UI to redraw, updating the status bar stats
            }
            Message::ButtonStart => {
                if self.state {
                    self.crawler.stop();
                } else {
                    self.crawler.start();
                }

                self.state = !self.state;
            }
            Message::ButtonScreenMain => {
                self.screen_active = Screen::Main;
            }
            Message::ButtonScreenTargets => {
                self.screen_active = Screen::Targets;
            }
            Message::IntensityChanged(intensity) => {
                self.intensity = intensity;
                self.crawler.set_intensity(intensity);
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(1000)).map(Message::Tick)
    }

    fn view(&mut self) -> Element<Message> {
        //TODO clean up this mess

        let stats = self.crawler.get_stats();
        let mut status = Row::new();

        status = status
            .push(Text::new(format!("Targets: {}", stats.targets)).size(12))
            .push(Space::with_width(Length::Units(15)));
        status = status
            .push(Text::new(format!("Online: {}", stats.online)).size(12))
            .push(Space::with_width(Length::Units(15)));
        status = status
            .push(Text::new(format!("Offline: {}", stats.offline)).size(12))
            .push(Space::with_width(Length::Units(15)));
        status = status
            .push(Text::new(format!("Requests: {}", stats.requests)).size(12))
            .push(Space::with_width(Length::Units(15)));
        status = status.push(Space::with_width(Length::Fill));

        let btn = match self.screen_active {
            Screen::Main => Button::new(
                &mut self.button_view_targets,
                Text::new("Show targets")
                    .size(10)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .padding(2)
            .min_width(100)
            .on_press(Message::ButtonScreenTargets)
            .style(style::Button::Secondary),
            Screen::Targets => Button::new(
                &mut self.button_view_main,
                Text::new("Back")
                    .size(10)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .padding(2)
            .min_width(100)
            .on_press(Message::ButtonScreenMain)
            .style(style::Button::Secondary),
        };
        status = status.push(btn);

        let screen = match self.screen_active {
            Screen::Main => {
                let (btn_style, btn_label) = if self.state {
                    (style::Button::Danger, "Stop")
                } else {
                    (style::Button::Success, "Start")
                };

                Column::new()
                    .spacing(20)
                    .push(Text::new("Resource limit"))
                    .push(Slider::new(
                        &mut self.slider_intensity,
                        0..=100,
                        self.intensity,
                        Message::IntensityChanged,
                    ))
                    .push(
                        Text::new(intensity_to_string(self.intensity))
                            .width(Length::Fill)
                            .horizontal_alignment(HorizontalAlignment::Center),
                    )
                    .push(
                        Button::new(
                            &mut self.button_start,
                            Text::new(btn_label).horizontal_alignment(HorizontalAlignment::Center),
                        )
                        .padding(12)
                        .min_width(100)
                        .on_press(Message::ButtonStart)
                        .style(btn_style),
                    )
            }
            _ => {
                let mut list = String::new();
                for itm in self.crawler.get_targets() {
                    list.push_str(&itm);
                    list.push('\n');
                }

                Column::new()
                    .spacing(20)
                    .push(Text::new("These are the current targets:"))
                    .push(Text::new(list).size(12))
                // .push(Column::new().height(Length::Units(2096)))
            }
        };

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(screen)
            .into();

        let scrollable = Scrollable::new(&mut self.scroll)
            .push(Container::new(content).width(Length::Fill).center_x());

        let header = Container::new(status)
            .width(Length::Fill)
            .center_x()
            .padding(5);

        Column::new().push(header).push(scrollable).into()
    }
}

/// Turns the intensity value into a human readable text
fn intensity_to_string(intensity: u8) -> &'static str {
    match intensity {
        0..=25 => "Your internet should barely feel slower",
        26..=50 => "Your internet should be okay",
        51..=75 => "Your internet may slow down",
        _ => "Your internet may become unusable",
    }
}
