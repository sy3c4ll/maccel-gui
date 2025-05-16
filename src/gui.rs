use crate::GuiTheme;
use iced::keyboard::{Key, Modifiers, key::Named, on_key_press};
use iced::widget::text_input;
use iced::{Element, Result, Task, Theme, application};
use maccel_core::{ALL_COMMON_PARAMS, ALL_LINEAR_PARAMS};
use maccel_core::{ALL_PARAMS, ContextRef, Param, persist::ParamStore};

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    FieldInput(Param, String),
    FieldUpdate(Param),
    NextField,
    PrevField,
    NextMode,
    PrevMode,
}

#[derive(Debug)]
pub struct Gui<PS: ParamStore> {
    context: ContextRef<PS>,
    input_buffer: [String; ALL_PARAMS.len()],
    focused: Option<Param>,
}

impl<PS: ParamStore + 'static> Gui<PS> {
    pub fn run(self) -> Result {
        application("maccel", Gui::update, Gui::view)
            .subscription(|_| on_key_press(Gui::<PS>::handle_key))
            .antialiasing(true)
            .centered()
            .theme(|_| Theme::CatppuccinMocha)
            .run_with(|| (self, Task::none()))
    }
    fn view(&self) -> Element<Message> {
        let theme = GuiTheme::new();
        (theme.screen)(self.context.clone(), &self.input_buffer)
    }
}

impl<PS: ParamStore> Gui<PS> {
    pub fn new(context: ContextRef<PS>) -> Self {
        let mut input_buffer = <[String; ALL_PARAMS.len()]>::default();
        for &param in ALL_PARAMS {
            input_buffer[param as usize] = context
                .get()
                .parameter(param)
                .map(|p| p.value.to_string())
                .unwrap_or_default();
        }
        Gui {
            context,
            input_buffer,
            focused: None,
        }
    }
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::FieldInput(param, s) => {
                self.input_buffer[param as usize] = s;
                self.focused = Some(param);
            }
            Message::FieldUpdate(param) => {
                if let Ok(f) = self.input_buffer[param as usize].parse::<f64>() {
                    self.context
                        .get_mut()
                        .update_param_value(param, f)
                        .expect("failed updating param");
                }
                self.input_buffer[param as usize] = self
                    .context
                    .get()
                    .parameter(param)
                    .map_or_else(String::new, |p| p.value.to_string());
            }
            Message::NextField => {
                if let Some(param) = self.focused {
                    let next = ALL_COMMON_PARAMS
                        .iter()
                        .chain(ALL_LINEAR_PARAMS)
                        .cycle()
                        .skip_while(|&&p| p != param)
                        .nth(1)
                        .unwrap();
                    self.focused = Some(*next);
                    return text_input::focus(next.name());
                }
            }
            Message::PrevField => {
                if let Some(param) = self.focused {
                    let prev = ALL_COMMON_PARAMS
                        .iter()
                        .chain(ALL_LINEAR_PARAMS)
                        .rev()
                        .cycle()
                        .skip_while(|&&p| p != param)
                        .nth(1)
                        .unwrap();
                    self.focused = Some(*prev);
                    return text_input::focus(prev.name());
                }
            }
            Message::NextMode => todo!(),
            Message::PrevMode => todo!(),
        }
        Task::none()
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
}
