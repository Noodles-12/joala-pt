mod widgets;

use iced::Element;

use crate::widgets::component::Component;

fn main() -> iced::Result {
    iced::run(Component::update, Component::view)
}

pub trait Action {
    fn view(&self) -> Element<'static, ActionMsg>;
    fn update(&mut self, message: ActionMsg);
}

#[derive(Debug, Clone, Copy)]
pub enum ActionMsg {
    // Counter
    Increment,
    Decrement,

    // Empty
    Add,
}

