use crate::Control;
use iced::alignment::{Horizontal, Vertical};
use iced::mouse::Cursor;
use iced::widget::canvas::fill::Rule;
use iced::widget::canvas::gradient::Linear;
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::path::lyon_path::geom::euclid::{Transform2D, Vector2D};
use iced::widget::canvas::{
    Fill, Frame, Geometry, Gradient, LineCap, LineDash, LineJoin, Path, Program, Stroke, Style,
    Text,
};
use iced::{Color, Pixels, Point, Rectangle, Renderer, Size, Theme, Vector, color};
use maccel_core::inputspeed::{read_input_speed, setup_input_speed_reader};
use maccel_core::{ContextRef, persist::ParamStore, sensitivity};

#[derive(Debug)]
pub struct Graph<PS: ParamStore> {
    context: ContextRef<PS>,
}

impl<PS: ParamStore> Graph<PS> {
    pub const AXIS_BOUNDS: Size = Size::new(80., 3.);
    pub const fn graph_area(size: Size) -> Rectangle {
        const ORIGIN_MARGIN: f32 = 40.;
        const EDGE_MARGIN: f32 = 20.;
        Rectangle {
            x: ORIGIN_MARGIN,
            y: size.height - ORIGIN_MARGIN,
            width: size.width - ORIGIN_MARGIN - EDGE_MARGIN,
            height: -size.height + ORIGIN_MARGIN + EDGE_MARGIN,
        }
    }

    pub fn new(context: ContextRef<PS>) -> Self {
        setup_input_speed_reader();
        Graph { context }
    }
    fn build_plots(&self, x_bld: &mut Builder, y_bld: &mut Builder, bounds: Rectangle) {
        const STEP_SZ: f32 = 0.25;

        let mut v = bounds.x;
        let (x_sens, y_sens) = sensitivity(
            v as f64,
            self.context.get().current_mode,
            &self.context.get().params_snapshot(),
        );
        let (x_sens, y_sens) = (x_sens as f32, y_sens as f32);
        x_bld.move_to(Point { x: v, y: x_sens });
        y_bld.move_to(Point { x: v, y: y_sens });
        v += STEP_SZ;

        while v <= bounds.x + bounds.width {
            let (x_sens, y_sens) = sensitivity(
                v as f64,
                self.context.get().current_mode,
                &self.context.get().params_snapshot(),
            );
            let (x_sens, y_sens) = (x_sens as f32, y_sens as f32);
            if (bounds.y..=bounds.y + bounds.height).contains(&x_sens) {
                x_bld.line_to(Point { x: v, y: x_sens });
            } else {
                x_bld.move_to(Point { x: v, y: x_sens });
            }
            if (bounds.y..=bounds.y + bounds.height).contains(&y_sens) {
                y_bld.line_to(Point { x: v, y: y_sens });
            } else {
                y_bld.move_to(Point { x: v, y: y_sens });
            }
            v += STEP_SZ;
        }
    }
}

impl<M, PS: ParamStore> Program<M> for Graph<PS> {
    type State = ();
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let axes = Graph::<PS>::AXIS_BOUNDS;
        let graph_area = Graph::<PS>::graph_area(bounds.size());
        let (graph_sz, graph_pos) = (graph_area.size(), graph_area.position());

        let graph_transform =
            Transform2D::scale(graph_sz.width / axes.width, graph_sz.height / axes.height)
                .then_translate(Vector2D::new(graph_pos.x, graph_pos.y));
        let x_plot_stroke = Stroke {
            style: Style::Solid(Color::WHITE),
            width: 1.,
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            line_dash: LineDash {
                segments: &[],
                offset: 0,
            },
        };
        let y_plot_stroke = Stroke {
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
        let indic_fill = Fill {
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
        let (x_indic, y_indic) = {
            let mut x_bld = Builder::new();
            let mut y_bld = Builder::new();
            self.build_plots(
                &mut x_bld,
                &mut y_bld,
                Rectangle {
                    x: 0.,
                    y: 0.,
                    width: input_speed,
                    height: axes.height,
                },
            );
            x_bld.line_to(Point {
                x: input_speed,
                y: 0.,
            });
            y_bld.line_to(Point {
                x: input_speed,
                y: 0.,
            });
            x_bld.line_to(Point::ORIGIN);
            y_bld.line_to(Point::ORIGIN);
            x_bld.close();
            y_bld.close();
            (
                x_bld.build().transform(&graph_transform),
                y_bld.build().transform(&graph_transform),
            )
        };
        frame.fill(&x_indic, indic_fill);
        frame.fill(&y_indic, indic_fill);

        let (x_plot, y_plot) = {
            let mut x_bld = Builder::new();
            let mut y_bld = Builder::new();
            self.build_plots(&mut x_bld, &mut y_bld, Rectangle::with_size(axes));
            (
                x_bld.build().transform(&graph_transform),
                y_bld.build().transform(&graph_transform),
            )
        };
        frame.stroke(&x_plot, x_plot_stroke);
        frame.stroke(&y_plot, y_plot_stroke);

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

        let controls = Control::controls(self.context.clone());
        let mut x_labels = (1..=axes.width as u32 / 10)
            .map(|u| u as f32 * 10.)
            .collect::<Vec<_>>();
        let mut y_labels = (1..=axes.height as u32 * 2)
            .map(|u| u as f32 / 2.)
            .collect::<Vec<_>>();
        x_labels.extend(controls.iter().map(|c| c.location().x));
        y_labels.extend(controls.iter().map(|c| c.location().y));
        x_labels.retain(|f| (0. ..=axes.width).contains(f));
        y_labels.retain(|f| (0. ..=axes.height).contains(f));
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
            size: Pixels(10.),
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
            size: Pixels(10.),
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
