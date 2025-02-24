#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Param {
    SensMult,
    Accel,
    Offset,
    OutputCap,
}

impl Param {
    pub const fn display_name(&self) -> &'static str {
        match self {
            Param::SensMult => "Sens-Multiplier",
            Param::Accel => "Accel",
            Param::Offset => "Offset",
            Param::OutputCap => "Output-Cap",
        }
    }
    pub const fn kernel_name(&self) -> &'static str {
        match self {
            Param::SensMult => "SENS_MULT",
            Param::Accel => "ACCEL",
            Param::Offset => "OFFSET",
            Param::OutputCap => "OUTPUT_CAP",
        }
    }
}
