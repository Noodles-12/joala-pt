use iced::widget::{button, text};
use iced::{Element, Length};
use iced::alignment::{Horizontal, Vertical};

use crate::{Action, ActionMsg};

#[derive(Default)]
pub struct Empty {}

impl Empty {
    pub fn new() -> Self {
        Empty {}
    }
}

impl Action for Empty {
    fn view(&self) -> Element<'static, ActionMsg> {
        button(text("+")
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .size(50)
        )
        .on_press(ActionMsg::Add)
        .width(Length::Fixed(156.0))
        .height(Length::Fixed(156.0))
        .into()
    }

    fn update(&mut self, message: ActionMsg) {
        match message {
            ActionMsg::Add => println!("Add Button Pressed"),
            _ => (),
        }
    } 
}