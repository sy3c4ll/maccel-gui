pub mod control;
pub mod graph;
pub mod gui;
mod mock;

pub use control::Control;
pub use graph::Graph;
pub use gui::Gui;
pub use mock::run_mock;

pub fn run_gui() -> iced::Result {
    use maccel_core::{ALL_PARAMS, ContextRef, TuiContext, persist::SysFsStore};
    Gui::new(ContextRef::new(TuiContext::new(SysFsStore, ALL_PARAMS))).run()
}
