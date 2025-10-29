mod widgets;

use crate::widgets::component::Component;

fn main() -> iced::Result {
    iced::application("joala-pt", Component::update, Component::view)
        .run()
}

