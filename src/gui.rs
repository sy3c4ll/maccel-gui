use crate::{graph::Graph, param::Param, params::Params};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{canvas, column, row, text, text_input};
use iced::Length::FillPortion;
use iced::{application, Element, Fill, Result, Task, Theme};
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    FieldInput(Param, String),
    FieldUpdate(Param),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InputBuffer {
    pub sens_mult: String,
    pub accel: String,
    pub offset: String,
    pub output_cap: String,
}

impl Index<Param> for InputBuffer {
    type Output = String;
    fn index(&self, param: Param) -> &Self::Output {
        match param {
            Param::SensMult => &self.sens_mult,
            Param::Accel => &self.accel,
            Param::Offset => &self.offset,
            Param::OutputCap => &self.output_cap,
        }
    }
}

impl IndexMut<Param> for InputBuffer {
    fn index_mut(&mut self, param: Param) -> &mut Self::Output {
        match param {
            Param::SensMult => &mut self.sens_mult,
            Param::Accel => &mut self.accel,
            Param::Offset => &mut self.offset,
            Param::OutputCap => &mut self.output_cap,
        }
    }
}

impl From<Params> for InputBuffer {
    fn from(params: Params) -> Self {
        InputBuffer {
            sens_mult: params.sens_mult.to_string(),
            accel: params.accel.to_string(),
            offset: params.offset.to_string(),
            output_cap: params.output_cap.to_string(),
        }
    }
}

impl Default for InputBuffer {
    fn default() -> Self {
        InputBuffer::from(Params::default())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Gui {
    params: Params,
    input_buffer: InputBuffer,
}

impl Gui {
    pub fn run(self) -> Result {
        application("maccel", Gui::update, Gui::view)
            .antialiasing(true)
            .centered()
            .theme(|_| Theme::TokyoNight)
            .run_with(|| (self, Task::none()))
    }
    fn update(&mut self, msg: Message) {
        match msg {
            Message::FieldInput(param, s) => self.input_buffer[param] = s,
            Message::FieldUpdate(param) => {
                if let Ok(f) = self.input_buffer[param].parse::<f64>() {
                    self.params[param] = f;
                }
                self.input_buffer[param] = self.params[param].to_string();
            }
        }
    }
    fn view(&self) -> Element<Message> {
        row![
            column![
                self.param_box(Param::SensMult),
                self.param_box(Param::Accel),
                self.param_box(Param::Offset),
                self.param_box(Param::OutputCap),
            ]
            .spacing(20.)
            .padding(10.)
            .align_x(Horizontal::Center)
            .width(FillPortion(1)),
            canvas(Graph::from(self.params))
                .width(FillPortion(3))
                .height(Fill),
        ]
        .align_y(Vertical::Center)
        .height(Fill)
        .into()
    }

    fn param_box(&self, param: Param) -> Element<'static, Message> {
        row![
            text(param.display_name())
                .align_x(Horizontal::Right)
                .width(Fill),
            text_input(param.kernel_name(), &self.input_buffer[param])
                .on_input(move |s| Message::FieldInput(param, s))
                .on_submit(Message::FieldUpdate(param))
                .align_x(Horizontal::Left)
                .width(Fill),
        ]
        .spacing(5.)
        .align_y(Vertical::Center)
        .width(Fill)
        .into()
    }
}

impl From<Params> for Gui {
    fn from(params: Params) -> Self {
        Gui {
            params,
            input_buffer: InputBuffer::from(params),
        }
    }
}
