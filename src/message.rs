use maccel_core::Param;

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    FieldInput(Param, String),
    FieldUpdate(Param),
    NextField,
    PrevField,
    NextMode,
    PrevMode,
}
