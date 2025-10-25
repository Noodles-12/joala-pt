use iced::Element;
use crate::widgets::empty::Empty;
use crate::widgets::counter::Counter;

#[derive(Default)]
pub struct Component {
    state: ComponentState,
}

impl Component {
    fn view(&self) -> Element<'static, ComponentMsg> {
        match self.state {
            ComponentState::Empty => Empty::new().view().map(|_| ComponentMsg::Create),
            ComponentState::Function => Counter::new().view().map(|_| ComponentMsg::Execute),
        }
    }

    fn update(&mut self, msg: ComponentMsg) {
        match msg {
            ComponentMsg::Create => self.state = ComponentState::Function,
            ComponentMsg::Execute =>  println!("Testing"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ComponentMsg {
    Create,
    Execute,
}

#[derive(Debug, Clone, Copy)]
pub enum ComponentState {
    Empty,
    Function,
}

impl Default for ComponentState {
    fn default() -> Self {
        Self::Empty
    }
}