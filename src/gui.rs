use crate::graph::Graph;
use iced::Length::FillPortion;
use iced::widget::container::Style;
use iced::widget::{Space, button, canvas, center, column, container, row, text, text_input};
use iced::{Border, Element, Fill, Result, Task, Theme, application};
use iced::{alignment::Horizontal, widget::scrollable};
use iced::{
    border::Radius,
    keyboard::{Key, Modifiers, key::Named, on_key_press},
};
use maccel_core::fixedptc::Fpt;
use maccel_core::{AllParamArgs, Param};
use std::mem::transmute_copy;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    FieldInput(Param, String),
    FieldUpdate(Param),
    NextField,
    PrevField,
    NextMode,
    PrevMode,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InputBuffer {
    pub sens_mult: String,
    pub accel: String,
    pub offset: String,
    pub output_cap: String,
}

impl InputBuffer {
    pub fn new(params: &AllParamArgs) -> Self {
        InputBuffer {
            sens_mult: params.sens_mult.to_string(),
            accel: params.accel.to_string(),
            offset: params.offset_linear.to_string(),
            output_cap: params.output_cap.to_string(),
        }
    }
}

impl Index<Param> for InputBuffer {
    type Output = String;
    fn index(&self, param: Param) -> &Self::Output {
        match param {
            Param::SensMult => &self.sens_mult,
            Param::Accel => &self.accel,
            Param::OffsetLinear => &self.offset,
            Param::OutputCap => &self.output_cap,
            _ => todo!(),
        }
    }
}

impl IndexMut<Param> for InputBuffer {
    fn index_mut(&mut self, param: Param) -> &mut Self::Output {
        match param {
            Param::SensMult => &mut self.sens_mult,
            Param::Accel => &mut self.accel,
            Param::OffsetLinear => &mut self.offset,
            Param::OutputCap => &mut self.output_cap,
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub struct Gui {
    params: AllParamArgs,
    input_buffer: InputBuffer,
    focused: Option<Param>,
}

impl Gui {
    const PARAM_ORDER: [Param; 4] = [
        Param::SensMult,
        Param::Accel,
        Param::OffsetLinear,
        Param::OutputCap,
    ];
    const fn at(&self, param: Param) -> &Fpt {
        match param {
            Param::SensMult => &self.params.sens_mult,
            Param::Accel => &self.params.accel,
            Param::OffsetLinear => &self.params.offset_linear,
            Param::OutputCap => &self.params.output_cap,
            _ => todo!(),
        }
    }
    const fn at_mut(&mut self, param: Param) -> &mut Fpt {
        match param {
            Param::SensMult => &mut self.params.sens_mult,
            Param::Accel => &mut self.params.accel,
            Param::OffsetLinear => &mut self.params.offset_linear,
            Param::OutputCap => &mut self.params.output_cap,
            _ => todo!(),
        }
    }

    pub fn new(params: &AllParamArgs) -> Self {
        Gui {
            params: unsafe { transmute_copy(params) },
            input_buffer: InputBuffer::new(params),
            focused: None,
        }
    }
    pub fn run(self) -> Result {
        application("maccel", Gui::update, Gui::view)
            .subscription(|_| on_key_press(Gui::handle_key))
            .antialiasing(true)
            .centered()
            .theme(|_| Theme::TokyoNight)
            .run_with(|| (self, Task::none()))
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::FieldInput(param, s) => {
                self.input_buffer[param] = s;
                self.focused = Some(param);
            }
            Message::FieldUpdate(param) => {
                if let Ok(f) = self.input_buffer[param].parse::<f64>() {
                    *self.at_mut(param) = Fpt::from(f);
                }
                self.input_buffer[param] = self.at(param).to_string();
            }
            Message::NextField => {
                if let Some(param) = self.focused {
                    let next = *Gui::PARAM_ORDER
                        .iter()
                        .cycle()
                        .skip_while(|&&p| p != param)
                        .nth(1)
                        .unwrap();
                    self.focused = Some(next);
                    return text_input::focus(next.name());
                }
            }
            Message::PrevField => {
                if let Some(param) = self.focused {
                    let prev = *Gui::PARAM_ORDER
                        .iter()
                        .rev()
                        .cycle()
                        .skip_while(|&&p| p != param)
                        .nth(1)
                        .unwrap();
                    self.focused = Some(prev);
                    return text_input::focus(prev.name());
                }
            }
            Message::NextMode => todo!(),
            Message::PrevMode => todo!(),
        }
        Task::none()
    }
    fn view(&self) -> Element<Message> {
        row![
            center(scrollable(
                column![
                    self.param_box(Param::SensMult),
                    self.param_box(Param::Accel),
                    self.param_box(Param::OffsetLinear),
                    self.param_box(Param::OutputCap),
                    button("Apply"),
                ]
                .spacing(20.)
                .align_x(Horizontal::Center)
                .width(Fill)
            ))
            .style(|theme: &Theme| Style {
                border: Border {
                    color: theme.palette().primary,
                    width: 1.,
                    radius: Radius::new(10.),
                },
                ..Style::default()
            })
            .padding(20.)
            .width(FillPortion(1))
            .height(Fill),
            center(canvas(Graph::new(&self.params)).width(Fill).height(Fill))
                .style(|theme: &Theme| Style {
                    border: Border {
                        color: theme.palette().primary,
                        width: 1.,
                        radius: Radius::new(10.),
                    },
                    ..Style::default()
                })
                .width(FillPortion(3)),
        ]
        .spacing(5.)
        .padding(5.)
        .into()
    }
    fn handle_key(key: Key, modi: Modifiers) -> Option<Message> {
        match key {
            Key::Named(Named::Tab) if modi == Modifiers::empty() => Some(Message::NextField),
            Key::Named(Named::ArrowDown) => Some(Message::NextField),
            Key::Named(Named::Tab) if modi == Modifiers::SHIFT => Some(Message::PrevField),
            Key::Named(Named::ArrowUp) => Some(Message::PrevField),
            Key::Named(Named::ArrowRight) => Some(Message::NextMode),
            Key::Named(Named::ArrowLeft) => Some(Message::PrevMode),
            _ => None,
        }
    }
    fn param_box(&self, param: Param) -> Element<'static, Message> {
        container(
            column![
                text(param.display_name())
                    .align_x(Horizontal::Center)
                    .width(Fill),
                row![
                    Space::with_width(FillPortion(1)),
                    text_input(param.name(), &self.input_buffer[param])
                        .id(param.name())
                        .on_input(move |s| Message::FieldInput(param, s))
                        .on_submit(Message::FieldUpdate(param))
                        .padding(5.)
                        .align_x(Horizontal::Left)
                        .width(FillPortion(4)),
                    Space::with_width(FillPortion(1)),
                ],
            ]
            .spacing(5.),
        )
        .style(|theme: &Theme| Style {
            border: Border {
                color: theme.extended_palette().secondary.strong.color,
                width: 2.,
                radius: Radius::new(5.),
            },
            ..Style::default()
        })
        .padding([15., 0.])
        .into()
    }
}
