use crate::numbers::lerp;
use esp_idf_hal::rmt::Pulse;
use rgb::{RGB8, RGBA8};

pub trait LerpableColor {
    fn lerp(&self, color_b: Self, t: f64) -> Self;
}

impl LerpableColor for RGB8 {
    #[allow(dead_code)]
    fn lerp(&self, color_b: RGB8, t: f64) -> RGB8 {
        let r = lerp(self.r, color_b.r, t);
        let g = lerp(self.g, color_b.g, t);
        let b = lerp(self.b, color_b.b, t);
        RGB8::new(r, g, b)
    }
}

impl LerpableColor for RGBA8 {
    #[allow(dead_code)]
    fn lerp(&self, color_b: RGBA8, t: f64) -> RGBA8 {
        let r = lerp(self.r, color_b.r, t);
        let g = lerp(self.g, color_b.g, t);
        let b = lerp(self.b, color_b.b, t);
        let a = lerp(self.a, color_b.a, t);
        RGBA8::new(r, g, b, a)
    }
}

pub trait WritableColor {
    fn get_pulses<'a>(&'a self, pulses: &'a (Pulse, Pulse, Pulse, Pulse)) -> Vec<&Pulse>;
}

impl WritableColor for RGB8 {
    #[allow(dead_code)]
    fn get_pulses<'a>(&'a self, pulses: &'a (Pulse, Pulse, Pulse, Pulse)) -> Vec<&Pulse> {
        let color: u32 = ((self.g as u32) << 16) | ((self.r as u32) << 8) | self.b as u32;
        let (t0h, t0l, t1h, t1l) = pulses;
        let mut pulses = vec![];
        for i in (0..24).rev() {
            let p = 2_u32.pow(i);
            let bit = p & color != 0;
            if bit {
                pulses.push(t1h);
                pulses.push(t1l);
            } else {
                pulses.push(t0h);
                pulses.push(t0l);
            }
        }
        pulses
    }
}

impl WritableColor for RGBA8 {
    #[allow(dead_code)]
    fn get_pulses<'a>(&'a self, pulses: &'a (Pulse, Pulse, Pulse, Pulse)) -> Vec<&Pulse> {
        let color: u32 = ((self.g as u32) << 24)
            | ((self.r as u32) << 16)
            | ((self.b as u32) << 8)
            | self.a as u32;
        let (t0h, t0l, t1h, t1l) = pulses;
        let mut pulses = vec![];
        for i in (0..32).rev() {
            let p = 2_u32.pow(i);
            let bit = p & color != 0;
            if bit {
                pulses.push(t1h);
                pulses.push(t1l);
            } else {
                pulses.push(t0h);
                pulses.push(t0l);
            }
        }
        pulses
    }
}
