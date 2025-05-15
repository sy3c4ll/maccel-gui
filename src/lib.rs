mod control;
mod graph;
mod gui;
mod mock;
mod theme;

pub use control::Control;
pub use graph::Graph;
pub use gui::{Gui, Message};
pub use mock::run_mock;
pub use theme::Theme;

pub fn run_gui() -> iced::Result {
    use maccel_core::{ALL_PARAMS, ContextRef, TuiContext, persist::SysFsStore};
    Gui::new(ContextRef::new(TuiContext::new(SysFsStore, ALL_PARAMS))).run()
}
