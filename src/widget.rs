use iced::widget::{container, Container};
use iced::Element;
use iced::{widget::{button, text, Column, column}};

#[derive(Default)]
pub struct Add {
    counter: Counter,
}

#[derive(Debug, Clone, Copy)]
pub enum AddMessage {
    Percentage(f32)
}

impl Add {
    pub fn view(&self) -> Container<'_, AddMessage> {
        container(
            self.counter.view().
                
        )
            .padding(20)
            .style(container::rounded_box)
            .into()
    }

    pub fn update(&mut self, message: AddMessage) {

    } 
}

#[derive(Default)]
struct Counter {
    value: i32,
}

impl Counter {
    pub fn view(&self) -> Column<'_, Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value).size(50),
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

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}