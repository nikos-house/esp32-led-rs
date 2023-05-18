use crate::color::LerpableColor;
use crate::layer;
use std::time::Instant;

pub trait AnimationRunner {
    fn get_progress(&mut self) -> f32;
}

pub trait Animatable<T: LerpableColor> {
    fn get_layer(&self, progress: f32) -> Vec<T>;
}

pub struct ShiftAnimation<T: LerpableColor + Clone> {
    original_layer: Vec<T>,
}

impl<T: LerpableColor + Clone> ShiftAnimation<T> {
    #[allow(dead_code)]
    pub fn new(layer: Vec<T>) -> Self {
        Self {
            original_layer: layer,
        }
    }

    #[allow(dead_code)]
    fn get_shift_offset(&self, progress: f32) -> f64 {
        let offset = progress as f64 * self.original_layer.len() as f64;
        (offset * 10.0).round() / 10.0
    }
}

impl<T: LerpableColor + Clone> Animatable<T> for ShiftAnimation<T> {
    #[allow(dead_code)]
    fn get_layer(&self, progress: f32) -> Vec<T> {
        let offset = self.get_shift_offset(progress);

        layer::shift(self.original_layer.clone(), offset)
    }
}
