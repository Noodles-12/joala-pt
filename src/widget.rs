use iced::widget::{container, Container};
use iced::Element;
use iced::{widget::{button, text, column}};

#[derive(Default)]
pub struct Add {
    counter: Counter,
}

#[derive(Debug, Clone, Copy)]
pub enum AddMessage {
    Counter(CounterMsg),
}

impl Add {
    pub fn view(&self) -> Container<'_, AddMessage> {
        container(
            self.counter.view().map(move |msg| { AddMessage::Counter(msg)})
        )
            .padding(20)
            .style(container::rounded_box)
    }

    pub fn update(&mut self, message: AddMessage) {
        match message {
            AddMessage::Counter(msg) => {
                self.counter.update(msg);
            }
        }
    } 
}

#[derive(Default)]
struct Counter {
    value: i32,
}

impl Counter {
    pub fn view(&self) -> Element<'_, CounterMsg> {
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