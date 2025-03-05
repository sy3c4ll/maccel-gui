mod graph;
mod gui;
mod inputspeed;
mod params;

use gui::Gui;

fn main() -> iced::Result {
    Gui::default().run()
}
