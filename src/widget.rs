use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, Container};
use iced::{Element, Length};
use iced::{widget::{button, text, column, row}};

static BOX_SIZE: u32 = 160;

#[derive(Default)]
pub struct Add {
    pub counter: Counter,
}

#[derive(Debug, Clone, Copy)]
pub enum AddMessage {
    Counter(CounterMsg),
    Add,
}

impl Add {
    pub fn view(&self) -> Container<'_, AddMessage> {
        println!("Testing");
        container(
            //self.counter.view().map(move |msg| { AddMessage::Counter(msg)})
            button("+")
                .on_press(AddMessage::Add)
                .width(Length::Fixed(BOX_SIZE as f32))
                .height(Length::Fixed(BOX_SIZE as f32)),
        )
            .padding(20)
            .style(container::rounded_box)
            .width(Length::Fixed(BOX_SIZE as f32))
            .height(Length::Fixed(BOX_SIZE as f32))
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
    }

    pub fn update(&mut self, message: AddMessage) {
        match message {
            AddMessage::Counter(msg) => {
                self.counter.update(msg);
            },
            AddMessage::Add => println!("Add Button Pressed")
        }
    } 
}

#[derive(Default)]
pub struct Counter {
    value: u32,
}

impl Counter {
    pub fn new(count: u32) -> Self {
        Counter {
            value: count
        }
    }
    pub fn view(&self) -> Element<'_, CounterMsg> {
        println!("Testing");
        column![
            button("+").on_press(CounterMsg::Increment),
            text(self.value).size(50),
            button("-").on_press(CounterMsg::Decrement),
        ].into()
    }

    pub fn update(&mut self, message: CounterMsg) {
        match message {
            CounterMsg::Increment => {
                self.value += 1;
            }
            CounterMsg::Decrement => {
                self.value -= 1;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CounterMsg {
    Increment,
    Decrement,
}

#[derive(Default)]
pub struct Grid<'a> {
    list: Vec<Element<'a, GridMsg>>
}

impl<'a> Grid<'a> {
    pub fn view(&self) {
        
    }

    pub fn update(&mut self, msg: GridMsg) {

    }

}

#[derive(Debug, Clone, Copy)]
pub enum GridMsg {

}