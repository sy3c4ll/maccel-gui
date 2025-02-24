use crate::params::Params;
use iced::mouse::Cursor;
use iced::widget::canvas::path::lyon_path::geom::euclid::{Transform2D, Vector2D};
use iced::widget::canvas::{Frame, Geometry, Path, Program, Stroke, Style};
use iced::{Color, Point, Rectangle, Renderer, Size, Theme, Vector};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Graph {
    params: Params,
}

impl Graph {
    const AXIS_BOUNDS: Size = Size::new(80., 3.);
    const fn graph_area(size: Size) -> Rectangle {
        const ORIGIN_MARGIN: f32 = 40.;
        const EDGE_MARGIN: f32 = 20.;
        Rectangle {
            x: ORIGIN_MARGIN,
            y: size.height - ORIGIN_MARGIN,
            width: size.width - ORIGIN_MARGIN - EDGE_MARGIN,
            height: -size.height + ORIGIN_MARGIN + EDGE_MARGIN,
        }
    }
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
        let axes = Graph::AXIS_BOUNDS;
        let graph_area = Graph::graph_area(bounds.size());
        let (graph_sz, graph_pos) = (graph_area.size(), graph_area.position());
        let graph_stroke = Stroke {
            style: Style::Solid(Color::WHITE),
            width: 3.,
            ..Stroke::default()
        };

        let mut frame = Frame::new(renderer, bounds.size());

        let graph = Path::new(|b| {
            let Params {
                sens_mult,
                accel,
                offset,
                output_cap,
            } = self.params;
            let (sens_mult, accel, offset, output_cap) = (
                sens_mult as f32,
                accel as f32,
                offset as f32,
                output_cap as f32,
            );
            b.move_to(Point {
                x: 0.,
                y: sens_mult,
            });
            b.line_to(Point {
                x: offset,
                y: sens_mult,
            });
            if output_cap <= 0. {
                b.line_to(Point {
                    x: axes.width,
                    y: sens_mult + (axes.width - offset) * accel,
                })
            } else if output_cap <= 1. {
                b.line_to(Point {
                    x: axes.width,
                    y: sens_mult,
                });
            } else {
                b.line_to(Point {
                    x: offset + (sens_mult * (output_cap - 1.)) / accel,
                    y: sens_mult * output_cap,
                });
            }
        });
        let real_graph = graph.transform(
            &Transform2D::scale(graph_sz.width / axes.width, graph_sz.height / axes.height)
                .then_translate(Vector2D::new(graph_pos.x, graph_pos.y)),
        );
        frame.stroke(&real_graph, graph_stroke);

        let x_axis = Path::line(
            graph_pos + Vector::new(-10., 0.),
            graph_pos + Vector::new(graph_sz.width, 0.),
        );
        let y_axis = Path::line(
            graph_pos + Vector::new(0., 10.),
            graph_pos + Vector::new(0., graph_sz.height),
        );
        frame.stroke(&x_axis, graph_stroke);
        frame.stroke(&y_axis, graph_stroke);

        vec![frame.into_geometry()]
    }
}

impl From<Params> for Graph {
    fn from(params: Params) -> Self {
        Graph { params }
    }
}
