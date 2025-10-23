use crate::widgets::counter;

use std::{clone, default};

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, Container, Column};
use iced::{Element, Length};
use iced::{widget::{button, text, column, row}};

#[derive(Default)]
pub struct Component {
    state: ComponentState,
}

impl Component {
    fn view(&self) -> Element<'_, ComponentMsg> {
        todo!()
    }

    fn update(&mut self, message: ComponentMsg) {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ComponentMsg {
    Create,
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
/*
pub struct Component {}

#[derive(Debug, Clone, Copy)]
pub enum ComponentMsg {

}

impl<'a> Component<'a> {
    pub fn view(&self) -> Element<'a, ComponentMsg> {
        self.current
    }
}

impl<'a> Default for Component<'a> {
    fn default() -> Self {
        Self {
            current: container(text("Default")).into(),
            fn_cmp: Counter::default(),
            empty_cmp: Empty::default(),
        }
    }
}
*/

#[derive(Default)]
pub struct Grid<'a> {
    list: Vec<Vec<Element<'a, GridMsg>>>
}

#[derive(Debug, Clone, Copy)]
pub enum GridMsg {

}

impl<'a> Grid<'a> {
    pub fn view(&self) {
        
    }

    pub fn update(&mut self, msg: GridMsg) {

    }
}