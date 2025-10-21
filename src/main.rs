mod widget;

use widget::Empty;

fn main() -> iced::Result {
    iced::run("A cool counter", Empty::update, Empty::view)
}

