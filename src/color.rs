use crate::numbers::lerp;
use esp_idf_hal::rmt::Pulse;
use rgb::{RGB8, RGBA8};
use smart_leds::White;
use ws2812_esp32_rmt_driver::RGBW8;

pub trait LerpableColor {
    fn lerp(&self, color_b: &Self, t: f64) -> Self;
}

impl LerpableColor for RGB8 {
    #[allow(dead_code)]
    fn lerp(&self, color_b: &RGB8, t: f64) -> RGB8 {
        let r = lerp(self.r, color_b.r, t);
        let g = lerp(self.g, color_b.g, t);
        let b = lerp(self.b, color_b.b, t);
        RGB8::new(r, g, b)
    }
}

impl LerpableColor for RGBA8 {
    #[allow(dead_code)]
    fn lerp(&self, color_b: &RGBA8, t: f64) -> RGBA8 {
        let r = lerp(self.r, color_b.r, t);
        let g = lerp(self.g, color_b.g, t);
        let b = lerp(self.b, color_b.b, t);
        let a = lerp(self.a, color_b.a, t);
        RGBA8::new(r, g, b, a)
    }
}

impl LerpableColor for RGBW8 {
    #[allow(dead_code)]
    fn lerp(&self, color_b: &RGBW8, t: f64) -> RGBW8 {
        let r = lerp(self.r, color_b.r, t);
        let g = lerp(self.g, color_b.g, t);
        let b = lerp(self.b, color_b.b, t);
        let a = lerp(self.a.0, color_b.a.0, t);
        RGBW8::from((r, g, b, White(a)))
    }
}
