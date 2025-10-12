mod widget;

use widget::Add;
use widget::Counter;

fn main() -> iced::Result {
    iced::run("A cool counter", Add::update, Add::view)
}

