// Linear interpolation
// t = 0.0 -> start
// t = 1.0 -> end
pub fn lerp(a: u8, b: u8, t: f64) -> u8 {
    (a as f64 * (1.0 - t) + b as f64 * t).round() as u8
}
