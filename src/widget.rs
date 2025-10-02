use iced::widget::{container, Container};

pub struct Add {}

pub enum AddMessage {
    Percentage(f32)
}

impl Add {
    pub fn view(&self) -> Container<'_, AddMessage> {
        container("Text")
            .padding(20)
            .style(container::rounded_box)
            .into()
    }

    pub fn update() {
        
    }
}