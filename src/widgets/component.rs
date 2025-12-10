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
                println!("Some component exists");
                container(component.view().map(|msg|ComponentMsg::Execute(msg)))
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
            },
            None => { 
                println!("component doesn't exist");
                container(Empty::new().view().map(|_| ComponentMsg::Create))
                    .width(Length::Fixed(BOX_SIZE))
                    .height(Length::Fixed(BOX_SIZE))
                    .into()
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