use iced::alignment::{Horizontal, Vertical};
use iced::border::Radius;
use iced::widget::container;
use iced::{Border, Element, Length};

use crate::widgets::empty::Empty;
use crate::widgets::counter::*;

use crate::{Action, ActionMsg};

const BOX_SIZE: f32 = 212.0;

#[derive(Default)]
pub struct Component {
    component: Option<Box<dyn Action>>,
}

impl Component {
    pub fn view(&self) -> Element<'_, ComponentMsg> {
        match &self.component {
            Some(component) => {
                let comp = component.view().map(|msg| ComponentMsg::Execute(msg));
                wrap_in_container(comp)
            },
            None => { 
                let empty = Empty::new().view().map(|_| ComponentMsg::Create);
                wrap_in_container(empty)
            }
        }
    }


    pub fn update(&mut self, msg: ComponentMsg) {
        match msg {
            ComponentMsg::Create => {
                let ctr = Counter::new();
                self.component = Some(Box::new(ctr));
            },
            ComponentMsg::Execute(msg) => {
                if let Some(mut comp) = self.component.take() {
                    comp.update(msg);
                    self.component = Some(comp);
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ComponentMsg {
    Create,
    Execute(ActionMsg),
}

fn wrap_in_container<'a>(component: Element<'a, ComponentMsg>) -> Element<'a, ComponentMsg> {
    container(component)
        .width(Length::Fixed(BOX_SIZE))
        .height(Length::Fixed(BOX_SIZE))
        .style(|_| container::Style {
            border: Border{
                width: 1.0,
                color: iced::Color::BLACK,
                radius: Radius::default(),
            },
            ..container::Style::default()
        })
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
}