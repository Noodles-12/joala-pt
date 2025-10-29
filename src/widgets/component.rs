use iced::widget::container;
use iced::{Element, Length};

use crate::widgets::empty::Empty;
use crate::widgets::counter::*;

const BOX_SIZE: f32 = 160.0;

#[derive(Default)]
pub struct Component<'a> {
    state: ComponentState<'a>,
}

impl<'a> Component<'a> {
    pub fn view(&self) -> Element<'_, ComponentMsg> {
        let content = match self.state {
            ComponentState::Empty => Empty::new().view().map(|_| ComponentMsg::Create),
            ComponentState::Function(cont) => cont,
        };

        container(content)
            .width(Length::Fixed(BOX_SIZE))
            .height(Length::Fixed(BOX_SIZE))
            .into()
    }

    pub fn update(&mut self, msg: ComponentMsg) {
        match msg {
            ComponentMsg::Create => self.state = ComponentState::Function(Counter::new().view().map(|msg| ComponentMsg::Execute(msg))),
            ComponentMsg::Execute(msg) =>  {
                
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ComponentMsg {
    Create,
    Execute(CounterMsg),
}

pub enum ComponentState<'a> {
    Empty,
    Function(Element<'a, ComponentMsg>),
}

impl<'a> Default for ComponentState<'a> {
    fn default() -> Self {
        Self::Empty
    }
}