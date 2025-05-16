use iced::alignment::{Horizontal, Vertical};
use iced::widget::canvas::fill::Rule;
use iced::widget::canvas::gradient::Linear;
use iced::widget::canvas::{Fill, Gradient, LineCap, LineDash, LineJoin, Stroke, Style, Text};
use iced::{Color, Pixels, Point, Rectangle, Size, Vector, color};

pub struct Theme {
    pub h_plot_stroke: Stroke<'static>,
    pub v_plot_stroke: Stroke<'static>,
    pub h_speedo_fill: Fill,
    pub v_speedo_fill: Fill,
    pub x_axis_stroke: Stroke<'static>,
    pub y_axis_stroke: Stroke<'static>,
    pub x_label_text: Box<dyn Fn(f32) -> Text>,
    pub y_label_text: Box<dyn Fn(f32) -> Text>,
}

impl Theme {
    pub fn from_bounds(axes: Size, area: Rectangle) -> Self {
        Theme {
            h_plot_stroke: Stroke {
                style: Style::Solid(Color::WHITE),
                width: 1.,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                line_dash: LineDash {
                    segments: &[],
                    offset: 0,
                },
            },
            v_plot_stroke: Stroke {
                style: Style::Solid(Color::WHITE),
                width: 1.,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                line_dash: LineDash {
                    segments: &[],
                    offset: 0,
                },
            },
            h_speedo_fill: Fill {
                style: Style::Gradient(Gradient::Linear(
                    Linear::new(
                        area.position(),
                        area.position()
                            + Vector {
                                x: area.width,
                                y: 0.,
                            },
                    )
                    .add_stop(0., color!(0x404000, 0.1))
                    .add_stop(0.5, color!(0x404000, 0.9)),
                )),
                rule: Rule::NonZero,
            },
            v_speedo_fill: Fill {
                style: Style::Gradient(Gradient::Linear(
                    Linear::new(
                        area.position(),
                        area.position()
                            + Vector {
                                x: area.width,
                                y: 0.,
                            },
                    )
                    .add_stop(0., color!(0x404000, 0.1))
                    .add_stop(0.5, color!(0x404000, 0.9)),
                )),
                rule: Rule::NonZero,
            },
            x_axis_stroke: Stroke {
                style: Style::Gradient(Gradient::Linear(
                    Linear::new(
                        area.position(),
                        area.position()
                            + Vector {
                                x: area.width,
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
            },
            y_axis_stroke: Stroke {
                style: Style::Gradient(Gradient::Linear(
                    Linear::new(
                        area.position(),
                        area.position()
                            + Vector {
                                x: 0.,
                                y: area.height,
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
            },
            x_label_text: Box::new(move |f| Text {
                content: f.to_string(),
                position: Point {
                    x: f * area.width / axes.width + area.x,
                    y: area.y + 10.,
                },
                color: Color::WHITE,
                size: Pixels(10.),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                ..Text::default()
            }),
            y_label_text: Box::new(move |f| Text {
                content: f.to_string(),
                position: Point {
                    x: area.x - 10.,
                    y: f * area.height / axes.height + area.y,
                },
                color: Color::WHITE,
                size: Pixels(10.),
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                ..Text::default()
            }),
        }
    }
}
