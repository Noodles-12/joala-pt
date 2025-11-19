use iced::widget::container;
use iced::{Element, Length};

use crate::widgets::empty::Empty;
use crate::widgets::counter::*;

use crate::{Action, ActionMsg};

const BOX_SIZE: f32 = 160.0;

#[derive(Default)]
pub struct Component {
    component: Option<Box<dyn Action>>,
    state: ComponentState,
}

impl Component {
    pub fn view(&self) -> Element<'_, ComponentMsg> {
        match &self.component {
            Some(component) => container(component.view().map(|msg|ComponentMsg::Execute(msg)))
                .width(Length::Fixed(BOX_SIZE))
                .height(Length::Fixed(BOX_SIZE))
                .into(),
            None => container(Empty::new().view().map(|_| ComponentMsg::Create))
                .width(Length::Fixed(BOX_SIZE))
                .height(Length::Fixed(BOX_SIZE))
                .into(),
        }
    }


    pub fn update(&mut self, msg: ComponentMsg) {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ComponentMsg {
    Create,
    Execute(ActionMsg),
}

pub enum ComponentState {
    Empty,
    Function,
}

impl Default for ComponentState {
    fn default() -> Self {
        Self::Empty
    }
}