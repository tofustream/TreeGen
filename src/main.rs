mod ui;
mod tree;
mod message;

use ui::TreeGen;

fn main() -> iced::Result {
    iced::run("TreeGen", TreeGen::update, TreeGen::view)
}
