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
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(CounterMsg::Increment),

            // We show the value of the counter here
            text(self.value).size(50),

            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
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