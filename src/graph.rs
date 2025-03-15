use crate::{inputspeed::read_input_speed, params::Params};
use iced::alignment::{Horizontal, Vertical};
use iced::mouse::Cursor;
use iced::widget::canvas::fill::Rule;
use iced::widget::canvas::gradient::Linear;
use iced::widget::canvas::path::lyon_path::geom::euclid::{Transform2D, Vector2D};
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{
    Fill, Frame, Geometry, Gradient, LineCap, LineDash, LineJoin, Path, Program, Stroke, Style,
    Text,
};
use iced::{color, Color, Pixels, Point, Rectangle, Renderer, Size, Theme, Vector};

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
    fn build_graph(&self, builder: &mut Builder, limit: Size) {
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

        builder.move_to(Point {
            x: 0.,
            y: sens_mult,
        });
        if output_cap <= 0. {
            // No cap
            if offset > 0. {
                builder.line_to(Point {
                    x: offset,
                    y: sens_mult,
                });
            }
            builder.line_to(Point {
                x: limit.width,
                y: sens_mult + (limit.width - offset) * accel,
            });
        } else if output_cap <= 1. {
            // Nonsensical cap, but treat as no accel
            builder.line_to(Point {
                x: limit.width,
                y: sens_mult,
            });
        } else {
            // Well-defined cap
            if offset > 0. {
                builder.line_to(Point {
                    x: offset,
                    y: sens_mult,
                });
            }
            if offset + (sens_mult * (output_cap - 1.)) / accel < limit.width {
                builder.line_to(Point {
                    x: offset + (sens_mult * (output_cap - 1.)) / accel,
                    y: sens_mult * output_cap,
                });
                builder.line_to(Point {
                    x: limit.width,
                    y: sens_mult * output_cap,
                });
            } else {
                builder.line_to(Point {
                    x: limit.width,
                    y: sens_mult + (limit.width - offset) * accel,
                });
            }
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

        let graph_transform =
            Transform2D::scale(graph_sz.width / axes.width, graph_sz.height / axes.height)
                .then_translate(Vector2D::new(graph_pos.x, graph_pos.y));
        let graph_stroke = Stroke {
            style: Style::Solid(Color::WHITE),
            width: 1.,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            line_dash: LineDash {
                segments: &[],
                offset: 0,
            },
        };
        let x_axis_stroke = Stroke {
            style: Style::Gradient(Gradient::Linear(
                Linear::new(
                    graph_pos,
                    graph_pos
                        + Vector {
                            x: graph_sz.width,
                            y: 0.,
                        },
                )
                .add_stop(0., color!(0x00ff00))
                .add_stop(1., color!(0xff0000)),
            )),
            width: 3.,
            line_cap: LineCap::Square,
            line_join: LineJoin::Bevel,
            line_dash: LineDash {
                segments: &[],
                offset: 0,
            },
        };
        let y_axis_stroke = Stroke {
            style: Style::Gradient(Gradient::Linear(
                Linear::new(
                    graph_pos,
                    graph_pos
                        + Vector {
                            x: 0.,
                            y: graph_sz.height,
                        },
                )
                .add_stop(0., color!(0x00ff00))
                .add_stop(1., color!(0xff0000)),
            )),
            width: 3.,
            line_cap: LineCap::Square,
            line_join: LineJoin::Bevel,
            line_dash: LineDash {
                segments: &[],
                offset: 0,
            },
        };
        let input_indicator_fill = Fill {
            style: Style::Gradient(Gradient::Linear(
                Linear::new(
                    graph_pos,
                    graph_pos
                        + Vector {
                            x: graph_sz.width,
                            y: 0.,
                        },
                )
                .add_stop(0., color!(0x404000, 0.1))
                .add_stop(0.5, color!(0x404000, 0.9)),
            )),
            rule: Rule::NonZero,
        };

        let mut frame = Frame::new(renderer, bounds.size());

        let input_speed = (read_input_speed() as f32).clamp(0., axes.width);
        let input_indicator = Path::new(|b| {
            b.move_to(Point::ORIGIN);
            b.line_to(Point {
                x: 0.,
                y: sens_mult,
            });
            self.build_graph(
                b,
                Size {
                    width: input_speed,
                    height: axes.height,
                },
            );
            b.line_to(Point {
                x: input_speed,
                y: 0.,
            });
            b.line_to(Point::ORIGIN);
        });
        let real_input_indicator = input_indicator.transform(&graph_transform);
        frame.fill(&real_input_indicator, input_indicator_fill);

        let graph = Path::new(|b| self.build_graph(b, axes));
        let real_graph = graph.transform(&graph_transform);
        frame.stroke(&real_graph, graph_stroke);

        let x_axis = Path::line(
            graph_pos + Vector::new(-10., 0.),
            graph_pos + Vector::new(graph_sz.width, 0.),
        );
        let y_axis = Path::line(
            graph_pos + Vector::new(0., 10.),
            graph_pos + Vector::new(0., graph_sz.height),
        );
        frame.stroke(&x_axis, x_axis_stroke);
        frame.stroke(&y_axis, y_axis_stroke);

        let vertex_labels = if output_cap <= 0. {
            if offset > 0. {
                (&[offset][..], &[sens_mult][..])
            } else {
                (&[][..], &[sens_mult][..])
            }
        } else if output_cap <= 1. {
            (&[][..], &[sens_mult][..])
        } else {
            let cap_x = offset + (sens_mult * (output_cap - 1.)) / accel;
            if cap_x < axes.width {
                if offset > 0. {
                    (
                        &[offset, cap_x][..],
                        &[sens_mult, sens_mult * output_cap][..],
                    )
                } else {
                    (&[cap_x][..], &[sens_mult, sens_mult * output_cap][..])
                }
            } else {
                if offset > 0. {
                    (&[offset][..], &[sens_mult][..])
                } else {
                    (&[][..], &[sens_mult][..])
                }
            }
        };
        let default_labels = (
            (1..=axes.width as u32 / 10)
                .map(|u| u as f32 * 10.)
                .collect::<Vec<_>>(),
            (1..=axes.height as u32 * 2)
                .map(|u| u as f32 / 2.)
                .collect::<Vec<_>>(),
        );
        let (mut x_labels, mut y_labels) = default_labels;
        x_labels.extend_from_slice(vertex_labels.0);
        y_labels.extend_from_slice(vertex_labels.1);
        x_labels.sort_by(f32::total_cmp);
        y_labels.sort_by(f32::total_cmp);
        x_labels.dedup();
        y_labels.dedup();
        let x_labels_text = x_labels.iter().map(|f| Text {
            content: f.to_string(),
            position: Point {
                x: f * graph_sz.width / axes.width + graph_pos.x,
                y: graph_pos.y + 10.,
            },
            color: Color::WHITE,
            size: Pixels(9.),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            ..Text::default()
        });
        let y_labels_text = y_labels.iter().map(|f| Text {
            content: f.to_string(),
            position: Point {
                x: graph_pos.x - 10.,
                y: f * graph_sz.height / axes.height + graph_pos.y,
            },
            color: Color::WHITE,
            size: Pixels(9.),
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            ..Text::default()
        });
        for label in x_labels_text {
            frame.fill_text(label);
        }
        for label in y_labels_text {
            frame.fill_text(label);
        }

        vec![frame.into_geometry()]
    }
}

impl From<Params> for Graph {
    fn from(params: Params) -> Self {
        Graph { params }
    }
}
