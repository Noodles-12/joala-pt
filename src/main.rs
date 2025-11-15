mod widgets;

use iced::Element;

use crate::widgets::component::Component;

fn main() -> iced::Result {
    iced::application("joala-pt", Component::update, Component::view)
        .run()
}

pub trait Action {
    fn view(&self) -> Element<'_, ActionMsg>;
}

#[derive(Debug, Clone, Copy)]
pub enum ActionMsg {
    Increment,
    Decrement,
}

