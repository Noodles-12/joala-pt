use iced::alignment::Horizontal;
use iced::widget::{column, button, text};
use iced::{Element, Length};

use crate::{Action, ActionMsg};

const BUTTON_WIDTH: f32 = 42.0;
const BUTTON_HEIGHT: f32 = 42.0;
const TEXT_SIZE: f32 = 20.0;

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
            button(text("+").size(TEXT_SIZE).center())
                .on_press(ActionMsg::Increment)
                .width(Length::Fixed(BUTTON_WIDTH))
                .height(Length::Fixed(BUTTON_HEIGHT)),
            text(self.value)
                .size(TEXT_SIZE),
            button(text("-").size(TEXT_SIZE).center())
                .on_press(ActionMsg::Decrement)
                .width(Length::Fixed(BUTTON_WIDTH))
                .height(Length::Fixed(BUTTON_HEIGHT)),
        ]
        .align_x(Horizontal::Center)
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