mod graph;
mod gui;

use gui::Gui;
use maccel_core::persist::SysFsStore;
use maccel_core::{ALL_PARAMS, TuiContext};

fn main() -> iced::Result {
    Gui::new(&TuiContext::new(SysFsStore, ALL_PARAMS).params_snapshot()).run()
}
