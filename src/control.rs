use iced::Point;
use maccel_core::{AccelMode, ContextRef, persist::ParamStore};

pub mod shorthand {
    pub use super::Control::{Angle as A, NormX as X, NormY as Y, Point as P};
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Control {
    Angle(f32, f32, f32), // (x, y, theta)
    Point(f32, f32),      // (x, y)
    NormX(f32),           // (x,)
    NormY(f32),           // (y,)
}

impl Control {
    pub fn controls<PS: ParamStore>(context: ContextRef<PS>) -> Vec<Self> {
        let (mode, params) = {
            let context = context.get();
            (context.current_mode, context.params_snapshot())
        };
        #[allow(unused_variables)]
        let (
            sens_mult,
            yx_ratio,
            input_dpi,
            accel,
            offset_linear,
            output_cap,
            decay_rate,
            offset_natural,
            limit,
            gamma,
            smooth,
            motivity,
            sync_speed,
        ) = (
            f64::from(params.sens_mult) as f32,
            f64::from(params.yx_ratio) as f32,
            f64::from(params.input_dpi) as f32,
            f64::from(params.accel) as f32,
            f64::from(params.offset_linear) as f32,
            f64::from(params.output_cap) as f32,
            f64::from(params.decay_rate) as f32,
            f64::from(params.offset_natural) as f32,
            f64::from(params.limit) as f32,
            f64::from(params.gamma) as f32,
            f64::from(params.smooth) as f32,
            f64::from(params.motivity) as f32,
            f64::from(params.sync_speed) as f32,
        );

        use shorthand::*;
        // TODO:
        // Linear - special case 0 < output_cap <= 1
        // Natural - add control for decay_rate
        // Synchronous - I have no idea how this works help me
        match mode {
            AccelMode::Linear => {
                let mut ret = vec![
                    P(offset_linear, sens_mult),
                    A(offset_linear, sens_mult, accel.atan()),
                ];
                if output_cap > 1. {
                    ret.push(P(
                        offset_linear + (sens_mult * (output_cap - 1.)) / accel,
                        sens_mult * output_cap,
                    ));
                }
                ret
            }
            AccelMode::Natural => vec![P(offset_natural, sens_mult), Y(sens_mult * limit)],
            AccelMode::Synchronous => vec![],
        }
    }
    pub fn location(&self) -> Point {
        use shorthand::*;
        match *self {
            A(x, y, _delta) => Point { x, y },
            P(x, y) => Point { x, y },
            X(x) => Point { x, y: 0. },
            Y(y) => Point { x: 0., y },
        }
    }
}

impl From<(f32, f32)> for Control {
    fn from((x, y): (f32, f32)) -> Self {
        Control::Point(x, y)
    }
}

impl From<[f32; 2]> for Control {
    fn from([x, y]: [f32; 2]) -> Self {
        Control::Point(x, y)
    }
}

impl From<Point> for Control {
    fn from(Point { x, y }: Point) -> Self {
        Control::Point(x, y)
    }
}
