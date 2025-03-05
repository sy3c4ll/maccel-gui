use rand::random_range;
use std::ops::RangeInclusive;

static mut INPUT_SPEED: f64 = 0.;

#[inline]
fn diff_range(speed: f64) -> RangeInclusive<f64> {
    let begin = -speed / 16.;
    let end = (80. - speed) / 16.;
    begin..=end
}
pub fn read_input_speed() -> f64 {
    unsafe {
        INPUT_SPEED += random_range(diff_range(INPUT_SPEED));
        INPUT_SPEED
    }
}
