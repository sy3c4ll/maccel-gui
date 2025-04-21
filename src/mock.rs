use crate::Gui;
use maccel_core::{
    ALL_PARAMS, AccelMode, ContextRef, Param, TuiContext, fixedptc::Fpt, persist::ParamStore,
};

static mut RUNTIME_ACCEL_MODE: AccelMode = AccelMode::Linear;

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
        // Reserve self.0 for when #77 is merged
        self.0 = unsafe { RUNTIME_ACCEL_MODE };
        self.1[param as u8 as usize] = value;
        Ok(())
    }
    fn get(&self, param: &Param) -> anyhow::Result<Fpt> {
        Ok(self.1[*param as u8 as usize].into())
    }

    fn set_current_accel_mode(mode: AccelMode) {
        unsafe {
            RUNTIME_ACCEL_MODE = mode;
        }
    }
    fn get_current_accel_mode() -> AccelMode {
        unsafe { RUNTIME_ACCEL_MODE }
    }
}

pub fn run_mock() -> iced::Result {
    Gui::new(ContextRef::new(TuiContext::new(
        RuntimeStore::default(),
        ALL_PARAMS,
    )))
    .run()
}
