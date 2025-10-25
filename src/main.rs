mod widgets;

use crate::widgets::empty::Empty;

fn main() -> iced::Result {
    iced::run("A cool counter", Empty::update, Empty::view)
}

