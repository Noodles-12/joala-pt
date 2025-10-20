mod widget;

use widget::Add;

fn main() -> iced::Result {
    iced::run("A cool counter", Add::update, Add::view)
}

