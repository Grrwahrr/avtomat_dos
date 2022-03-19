use iced::{window, Application, Settings};
use ui::UserInterface;

mod crawler;
mod targets;
mod ui;

fn main() -> iced::Result {
    UserInterface::run(Settings {
        window: window::Settings {
            size: (600, 300),
            // min_size: None,
            // max_size: None,
            // resizable: false,
            // decorations: false,
            // transparent: false,
            // always_on_top: false,
            // icon: None
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
