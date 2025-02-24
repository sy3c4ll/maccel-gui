mod graph;
mod gui;
mod param;
mod params;

use gui::Gui;

fn main() -> iced::Result {
    Gui::default().run()
}
