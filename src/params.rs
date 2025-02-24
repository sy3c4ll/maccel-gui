use crate::param::Param;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Params {
    pub sens_mult: f64,
    pub accel: f64,
    pub offset: f64,
    pub output_cap: f64,
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
