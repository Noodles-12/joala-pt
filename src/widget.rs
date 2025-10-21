use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, Container, Column};
use iced::{Element, Length};
use iced::{widget::{button, text, column, row}};

static BOX_SIZE: u32 = 160;

#[derive(Default)]
pub struct Empty {}

#[derive(Debug, Clone, Copy)]
pub enum EmptyMsg {
    Add,
}

impl Empty {
    pub fn view(&self) -> Element<'_, EmptyMsg> {
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

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Counter {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),

            // We show the value of the counter here
            text(self.value).size(50),

            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}

pub struct Component<'a> {
    current: Element<'a, ComponentMsg>,
    fn_cmp: Counter,
    empty_cmp: Empty,
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
            current: container(text("Default")).into(),
            fn_cmp: Counter { value: 0 },
            empty_cmp: Empty {},
        }
    }
}