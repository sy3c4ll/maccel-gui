use crate::{Graph, Message};
use iced::alignment::{Horizontal, Vertical};
use iced::border::Radius;
use iced::widget::canvas::fill::Rule;
use iced::widget::canvas::gradient::Linear;
use iced::widget::canvas::{Fill, Gradient, LineCap, LineDash, LineJoin, Stroke, Style, Text};
use iced::widget::{
    Space, canvas, center, column, container, keyed_column, row, scrollable, text, text_input,
};
use iced::{
    Alignment, Border, Color, Element, Length, Pixels, Point, Rectangle, Size, Theme, Vector, color,
};
use maccel_core::ContextRef;
use maccel_core::{
    ALL_COMMON_PARAMS, ALL_LINEAR_PARAMS, ALL_NATURAL_PARAMS, ALL_SYNCHRONOUS_PARAMS, AccelMode,
    Param, persist::ParamStore,
};

pub struct GuiTheme<PS: ParamStore> {
    pub param_box: Box<dyn Fn(Param, &str) -> Element<'static, Message>>,
    pub wrapper_style: Box<dyn Fn(&Theme) -> container::Style>,
    pub params_div: Box<dyn Fn(AccelMode, &[String]) -> Element<'static, Message>>,
    pub graph_div: Box<dyn Fn(ContextRef<PS>) -> Element<'static, Message>>,
    pub screen: Box<dyn Fn(ContextRef<PS>, &[String]) -> Element<'static, Message>>,
}

impl<PS: ParamStore + 'static> GuiTheme<PS> {
    pub fn new() -> Self {
        let param_box = |param: Param, buf: &str| {
            container(
                column![
                    text(param.display_name())
                        .align_x(Horizontal::Center)
                        .width(Length::Fill),
                    row![
                        Space::with_width(Length::FillPortion(1)),
                        text_input(param.name(), buf)
                            .id(param.name())
                            .on_input(move |s| Message::FieldInput(param, s))
                            .on_submit(Message::FieldUpdate(param))
                            .padding(5.)
                            .align_x(Horizontal::Left)
                            .width(Length::FillPortion(4)),
                        Space::with_width(Length::FillPortion(1)),
                    ],
                ]
                .spacing(5.),
            )
            .style(|theme: &Theme| container::Style {
                border: Border {
                    color: theme.extended_palette().secondary.strong.color,
                    width: 2.,
                    radius: Radius::new(5.),
                },
                ..container::Style::default()
            })
            .padding([15., 0.])
            .into()
        };
        let wrapper_style = |theme: &Theme| container::Style {
            border: Border {
                color: theme.palette().primary,
                width: 1.,
                radius: Radius::new(10.),
            },
            ..container::Style::default()
        };
        let params_div = move |mode: AccelMode, bufs: &[String]| {
            center(scrollable(
                keyed_column(
                    ALL_COMMON_PARAMS
                        .iter()
                        .chain(match mode {
                            AccelMode::Linear => ALL_LINEAR_PARAMS,
                            AccelMode::Natural => ALL_NATURAL_PARAMS,
                            AccelMode::Synchronous => ALL_SYNCHRONOUS_PARAMS,
                        })
                        .map(|&p| (p, param_box(p, &bufs[p as usize]))),
                )
                .spacing(20.)
                .align_items(Alignment::Center)
                .width(Length::Fill),
            ))
            .style(wrapper_style)
            .padding(20.)
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .into()
        };
        let graph_div = move |ctx: ContextRef<PS>| {
            center(
                canvas(Graph::new(ctx))
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .style(wrapper_style)
            .width(Length::FillPortion(3))
            .into()
        };
        let screen = move |ctx: ContextRef<PS>, bufs: &[String]| {
            row![params_div(AccelMode::Linear, bufs), graph_div(ctx),]
                .spacing(5.)
                .padding(5.)
                .into()
        };
        GuiTheme {
            param_box: Box::new(param_box),
            wrapper_style: Box::new(wrapper_style),
            params_div: Box::new(params_div),
            graph_div: Box::new(graph_div),
            screen: Box::new(screen),
        }
    }
}

pub struct GraphTheme {
    pub h_plot_stroke: Stroke<'static>,
    pub v_plot_stroke: Stroke<'static>,
    pub h_speedo_fill: Fill,
    pub v_speedo_fill: Fill,
    pub x_axis_stroke: Stroke<'static>,
    pub y_axis_stroke: Stroke<'static>,
    pub x_label_text: Box<dyn Fn(f32) -> Text>,
    pub y_label_text: Box<dyn Fn(f32) -> Text>,
}

impl GraphTheme {
    pub fn new(axes: Size, area: Rectangle) -> Self {
        GraphTheme {
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
