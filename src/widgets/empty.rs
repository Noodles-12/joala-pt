use iced::widget::{button, text};
use iced::{Element};
use iced::alignment::{Horizontal, Vertical};

use crate::{Action, ActionMsg};

static BOX_SIZE: u32 = 160;

#[derive(Default)]
pub struct Empty {}

#[derive(Debug, Clone, Copy)]
pub enum EmptyMsg {
    Add,
}

impl Empty {
    pub fn new() -> Self {
        Empty {}
    }

    pub fn update(&mut self, message: ActionMsg) {
        match message {
            ActionMsg::Add => println!("Add Button Pressed"),
            _ => (),
        }
    } 
}

impl Action for Empty {
    fn view(&self) -> Element<'static, ActionMsg> {
        button(text("+")
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
        )
        .on_press(ActionMsg::Add)
        .into()
    }
}