use crate::Gui;
use maccel_core::{
    ALL_PARAMS, AccelMode, ContextRef, Param, TuiContext, fixedptc::Fpt, persist::ParamStore,
};

#[derive(Debug)]
struct RuntimeStore(AccelMode, [f64; ALL_PARAMS.len()]);

impl Default for RuntimeStore {
    fn default() -> Self {
        RuntimeStore(
            AccelMode::Linear,
            [1., 1., 1000., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.],
        )
    }
}

impl ParamStore for RuntimeStore {
    fn set(&mut self, param: Param, value: f64) -> anyhow::Result<()> {
        self.1[param as usize] = value;
        Ok(())
    }
    fn get(&self, param: Param) -> anyhow::Result<Fpt> {
        Ok(self.1[param as usize].into())
    }

    fn set_current_accel_mode(&mut self, mode: AccelMode) -> anyhow::Result<()> {
        self.0 = mode;
        Ok(())
    }
    fn get_current_accel_mode(&self) -> anyhow::Result<AccelMode> {
        Ok(self.0)
    }
}

pub fn run_mock() -> iced::Result {
    Gui::new(ContextRef::new(TuiContext::new(
        RuntimeStore::default(),
        ALL_PARAMS,
    )))
    .run()
}
