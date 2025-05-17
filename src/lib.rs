mod app;
mod control;
mod graph;
mod graph_theme;
mod gui;
mod message;
mod mock;

pub use app::App;
pub use control::Control;
pub use graph::Graph;
pub use graph_theme::GraphTheme;
pub use gui::{DefaultGui, Gui};
pub use message::Message;
pub use mock::run_mock;

pub fn run_gui() -> iced::Result {
    use maccel_core::{ALL_PARAMS, ContextRef, TuiContext, persist::SysFsStore};
    App::new(ContextRef::new(TuiContext::new(SysFsStore, ALL_PARAMS))).run::<DefaultGui>()
}
