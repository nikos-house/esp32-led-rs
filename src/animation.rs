use crate::color::LerpableColor;
use crate::layer;

pub trait AnimationRunner {
    fn get_progress(&mut self) -> f32;
}

pub(crate) trait Animatable<T: LerpableColor + Clone> {
    fn get_layer(&mut self) -> Vec<T>;
}

#[derive(Clone)]
pub struct ShiftAnimation<R: AnimationRunner + Clone, T: LerpableColor + Clone> {
    runner: R,
    original_layer: Vec<T>,
    last_progress: f32,
}

impl<R: AnimationRunner + Clone, T: LerpableColor + Clone> ShiftAnimation<R, T> {
    #[allow(dead_code)]
    pub fn new(runner: R, layer: Vec<T>) -> Self {
        Self {
            runner,
            original_layer: layer,
            last_progress: 0.0,
        }
    }

    #[allow(dead_code)]
    fn get_shift_offset(&mut self) -> f64 {
        let progress = self.runner.get_progress();
        self.last_progress = progress;
        let offset = progress as f64 * self.original_layer.len() as f64;
        (offset * 10.0).round() / 10.0
    }
}

impl<R: AnimationRunner + Clone, T: LerpableColor + Clone> Animatable<T> for ShiftAnimation<R, T> {
    #[allow(dead_code)]
    fn get_layer(&mut self) -> Vec<T> {
        let offset = self.get_shift_offset();

        layer::shift(self.original_layer.clone(), offset)
    }
}
