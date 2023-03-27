use crate::numbers::lerp;
use smart_leds::RGB8;

pub fn lerp_rgb(a: RGB8, b: RGB8, t: f64) -> RGB8 {
    let r = lerp(a.r, b.r, t);
    let g = lerp(a.g, b.g, t);
    let b = lerp(a.b, b.b, t);
    RGB8::new(r, g, b)
}
