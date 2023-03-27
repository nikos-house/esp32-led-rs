use crate::layer;
use smart_leds::RGB8;

pub trait AnimationRunner {
    fn get_progress(&mut self) -> f32;
}

pub(crate) trait Animatable {
    fn get_layer(&mut self) -> Vec<RGB8>;
}

pub struct ShiftAnimation<R: AnimationRunner> {
    runner: R,
    original_layer: Vec<RGB8>,
    last_progress: f32,
}

impl<R: AnimationRunner> ShiftAnimation<R> {
    pub fn new(runner: R, layer: Vec<RGB8>) -> Self {
        Self {
            runner,
            original_layer: layer.clone(),
            last_progress: 0.0,
        }
    }

    fn get_shift_offset(&mut self) -> f64 {
        let progress = self.runner.get_progress();
        self.last_progress = progress;
        let offset = progress as f64 * self.original_layer.len() as f64;
        (offset * 10.0).round() / 10.0
    }
}

impl<R: AnimationRunner> Animatable for ShiftAnimation<R> {
    fn get_layer(&mut self) -> Vec<RGB8> {
        let offset = self.get_shift_offset();

        layer::shift(self.original_layer.clone(), offset)
    }
}
