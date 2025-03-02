use std::ops::{Index, IndexMut};

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Params {
    pub sens_mult: f64,
    pub accel: f64,
    pub offset: f64,
    pub output_cap: f64,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            sens_mult: 1.,
            accel: 0.,
            offset: 0.,
            output_cap: 0.,
        }
    }
}

impl Index<Param> for Params {
    type Output = f64;
    fn index(&self, param: Param) -> &Self::Output {
        match param {
            Param::SensMult => &self.sens_mult,
            Param::Accel => &self.accel,
            Param::Offset => &self.offset,
            Param::OutputCap => &self.output_cap,
        }
    }
}

impl IndexMut<Param> for Params {
    fn index_mut(&mut self, param: Param) -> &mut Self::Output {
        match param {
            Param::SensMult => &mut self.sens_mult,
            Param::Accel => &mut self.accel,
            Param::Offset => &mut self.offset,
            Param::OutputCap => &mut self.output_cap,
        }
    }
}
