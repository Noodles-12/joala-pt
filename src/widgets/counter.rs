use iced::widget::{column, button, text};
use iced::Element;

#[derive(Default)]
pub struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum CounterMsg {
    Increment,
    Decrement,
}

impl Counter {
    pub fn new() -> Self {
        Counter { value: 0 }
    }
    pub fn view(&self) -> Element<'static, CounterMsg> {
        column![
            button("+").on_press(CounterMsg::Increment),
            text(self.value).size(50),
            button("-").on_press(CounterMsg::Decrement),
        ]
        .into()
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