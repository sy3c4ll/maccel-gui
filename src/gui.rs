use crate::Message;
use iced::alignment::Horizontal;
use iced::border::Radius;
use iced::widget::canvas::Program;
use iced::widget::{
    Space, canvas, center, column, container, keyed_column, row, scrollable, text, text_input,
};
use iced::{Alignment, Border, Element, Length, Theme};
use maccel_core::{
    ALL_COMMON_PARAMS, ALL_LINEAR_PARAMS, ALL_NATURAL_PARAMS, ALL_SYNCHRONOUS_PARAMS, AccelMode,
    Param,
};

pub trait Gui: 'static {
    fn param_box(param: Param, buf: &str) -> Element<'static, Message> {
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
    }
    fn wrapper_style(theme: &Theme) -> container::Style {
        container::Style {
            border: Border {
                color: theme.palette().primary,
                width: 1.,
                radius: Radius::new(10.),
            },
            ..container::Style::default()
        }
    }
    fn params_div(mode: AccelMode, bufs: &[String]) -> Element<'static, Message> {
        center(scrollable(
            keyed_column(
                ALL_COMMON_PARAMS
                    .iter()
                    .chain(match mode {
                        AccelMode::Linear => ALL_LINEAR_PARAMS,
                        AccelMode::Natural => ALL_NATURAL_PARAMS,
                        AccelMode::Synchronous => ALL_SYNCHRONOUS_PARAMS,
                    })
                    .map(|&p| (p, Self::param_box(p, &bufs[p as usize]))),
            )
            .spacing(20.)
            .align_items(Alignment::Center)
            .width(Length::Fill),
        ))
        .style(Self::wrapper_style)
        .padding(20.)
        .width(Length::FillPortion(1))
        .height(Length::Fill)
        .into()
    }
    fn graph_div(graph: impl Program<Message> + 'static) -> Element<'static, Message> {
        center(canvas(graph).width(Length::Fill).height(Length::Fill))
            .style(Self::wrapper_style)
            .width(Length::FillPortion(3))
            .into()
    }
    fn screen(
        graph: impl Program<Message> + 'static,
        bufs: &[String],
    ) -> Element<'static, Message> {
        row![
            Self::params_div(AccelMode::Linear, bufs),
            Self::graph_div(graph),
        ]
        .spacing(5.)
        .padding(5.)
        .into()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DefaultGui;

impl Gui for DefaultGui {}
