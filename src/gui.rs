use crate::Graph;
use iced::Length::FillPortion;
use iced::alignment::Horizontal;
use iced::border::Radius;
use iced::keyboard::{Key, Modifiers, key::Named, on_key_press};
use iced::widget::{Space, canvas, center, column, container, row, scrollable, text, text_input};
use iced::{Border, Element, Fill, Result, Task, Theme, application};
use maccel_core::{AllParamArgs, ContextRef, Param, persist::ParamStore};
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
pub struct Gui<PS: ParamStore> {
    context: ContextRef<PS>,
    input_buffer: InputBuffer,
    focused: Option<Param>,
}

impl<PS: ParamStore + 'static> Gui<PS> {
    pub fn run(self) -> Result {
        application("maccel", Gui::update, Gui::view)
            .subscription(|_| on_key_press(Gui::<PS>::handle_key))
            .antialiasing(true)
            .centered()
            .theme(|_| Theme::TokyoNight)
            .run_with(|| (self, Task::none()))
    }
}

impl<PS: ParamStore> Gui<PS> {
    const PARAM_ORDER: [Param; 4] = [
        Param::SensMult,
        Param::Accel,
        Param::OffsetLinear,
        Param::OutputCap,
    ];

    pub fn new(context: ContextRef<PS>) -> Self {
        let params = context.get().params_snapshot();
        Gui {
            context,
            input_buffer: InputBuffer::new(&params),
            focused: None,
        }
    }
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::FieldInput(param, s) => {
                self.input_buffer[param] = s;
                self.focused = Some(param);
            }
            Message::FieldUpdate(param) => {
                if let Ok(f) = self.input_buffer[param].parse::<f64>() {
                    self.context
                        .get_mut()
                        .update_param_value(param, f)
                        .expect("failed updating param");
                }
                self.input_buffer[param] = self
                    .context
                    .get()
                    .parameter(param)
                    .map_or_else(String::new, |p| p.value.to_string());
            }
            Message::NextField => {
                if let Some(param) = self.focused {
                    let next = *Gui::<PS>::PARAM_ORDER
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
                    let prev = *Gui::<PS>::PARAM_ORDER
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
                ]
                .spacing(20.)
                .align_x(Horizontal::Center)
                .width(Fill)
            ))
            .style(|theme: &Theme| container::Style {
                border: Border {
                    color: theme.palette().primary,
                    width: 1.,
                    radius: Radius::new(10.),
                },
                ..container::Style::default()
            })
            .padding(20.)
            .width(FillPortion(1))
            .height(Fill),
            center(
                canvas(Graph::new(self.context.clone()))
                    .width(Fill)
                    .height(Fill)
            )
            .style(|theme: &Theme| container::Style {
                border: Border {
                    color: theme.palette().primary,
                    width: 1.,
                    radius: Radius::new(10.),
                },
                ..container::Style::default()
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
}
