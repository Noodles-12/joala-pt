use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, Container};
use iced::{Element, Length};
use iced::{widget::{button, text, column, row}};

static BOX_SIZE: u32 = 160;

#[derive(Default)]
pub struct Add {}

#[derive(Debug, Clone, Copy)]
pub enum AddMessage {
    Add,
}

impl Add {
    pub fn view(&self) -> Element<'_, AddMessage> {
        button(text("+")
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
        )
        .on_press(AddMessage::Add)
        .into()
    }

    pub fn update(&mut self, message: AddMessage) {
        match message {
            AddMessage::Add => println!("Add Button Pressed")
        }
    } 
}

pub struct Component<'a> {
    current: Element<'a, ComponentMsg>,
}

#[derive(Debug, Clone, Copy)]
pub enum ComponentMsg {

}

#[derive(Default)]
pub struct Grid<'a> {
    list: Vec<Vec<Element<'a, GridMsg>>>
}

#[derive(Debug, Clone, Copy)]
pub enum GridMsg {

}

impl<'a> Grid<'a> {
    pub fn view(&self) {
        
    }

    pub fn update(&mut self, msg: GridMsg) {

    }

}

impl<'a> Default for Component<'a> {
    fn default() -> Self {
        Self {
            current: container(text("Default")).into()
        }
    }
}