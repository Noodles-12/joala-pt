mod widget;

use iced::{widget::{button, text, Column, column}};


fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}

#[derive(Default)]
struct Counter {
    value: i32,
}

impl Counter {
    pub fn view(&'_ self) -> Column<'_, Message> {
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

