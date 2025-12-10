use iced::widget::{column, button, text};
use iced::{Element, Length};

use crate::{Action, ActionMsg};

#[derive(Default)]
pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new() -> Self {
        Counter { value: 0 }
    }
}

impl Action for Counter {
    fn view(&self) -> Element<'static, ActionMsg> {
        column![
            button("+").on_press(ActionMsg::Increment)
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(10.0)),
            text(self.value).size(20),
            button("-").on_press(ActionMsg::Decrement)
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(10.0)),
        ]
        .into()
    }

    fn update(&mut self, message: ActionMsg) {
        match message {
            ActionMsg::Increment => {
                self.value += 1;
            }
            ActionMsg::Decrement => {
                self.value -= 1;
            },
            _ => (),
        }
    }
}