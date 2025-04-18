pub mod graph;
pub mod gui;

pub use graph::Graph;
pub use gui::Gui;

pub fn run_gui() -> iced::Result {
    use maccel_core::{ALL_PARAMS, ContextRef, TuiContext, persist::SysFsStore};
    Gui::new(ContextRef::new(TuiContext::new(SysFsStore, ALL_PARAMS))).run()
}
