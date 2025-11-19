use iced::widget::{column, button, text};
use iced::Element;

use crate::{Action, ActionMsg};

#[derive(Default)]
pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new() -> Self {
        Counter { value: 0 }
    }

    pub fn update(&mut self, message: ActionMsg) {
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

impl Action for Counter {
    fn view(&self) -> Element<'static, ActionMsg> {
        column![
            button("+").on_press(ActionMsg::Increment),
            text(self.value).size(50),
            button("-").on_press(ActionMsg::Decrement),
        ]
        .into()
    }
}