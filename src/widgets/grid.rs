// This will be the main page of the application
use crate::{ActionMsg, widgets::component::Component};
use iced::Element;

struct Grid {
    grid: Vec<Component>,
}

impl Grid {
    pub fn view(&self) -> Element<'_, GridMsg> {
        for i in 0..5 {
            
        }
        todo!()
    }

    pub fn update(&mut self, msg: GridMsg) {
        todo!()
    }

    
}

#[derive(Debug, Clone, Copy)]
enum GridMsg {
    TestAction(usize, ActionMsg)
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            grid: (0..15).map(|_| Component::default()).collect(),
        }
    }
}