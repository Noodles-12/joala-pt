use iced::widget::{button, text};
use iced::{Element};
use iced::alignment::{Horizontal, Vertical};

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

    pub fn view(&self) -> Element<'static, EmptyMsg> {
        button(text("+")
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
        )
        .on_press(EmptyMsg::Add)
        .into()
    }

    pub fn update(&mut self, message: EmptyMsg) {
        match message {
            EmptyMsg::Add => println!("Add Button Pressed")
        }
    } 
}