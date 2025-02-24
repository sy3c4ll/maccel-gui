use crate::params::Params;
use iced::mouse::Cursor;
use iced::widget::canvas::{Frame, Geometry, Path, Program};
use iced::{Color, Rectangle, Renderer, Theme};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Graph {
    params: Params,
}

impl<M> Program<M> for Graph {
    type State = ();
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let circle = Path::circle(frame.center(), 50.);
        frame.fill(&circle, Color::BLACK);
        vec![frame.into_geometry()]
    }
}

impl From<Params> for Graph {
    fn from(params: Params) -> Self {
        Graph { params }
    }
}
