use crate::updatable::Updatable;
use esp_idf_hal::gpio::{Input, InputPin, PinDriver};
use esp_idf_sys::EspError;
use observable_rs::Observable;
use smart_leds::RGB;

pub struct Cycle<T> {
    current_index: usize,
    options: Vec<T>,
    current_observable: Observable<T>,
}

impl<T: Clone> Cycle<T> {
    pub fn new(options: Vec<T>) -> Self {
        let current_index = 0;
        let current_observable = Observable::new(options[current_index].clone());
        Self {
            current_index,
            options,
            current_observable,
        }
    }

    pub fn next(&mut self) {
        self.current_index = (self.current_index + 1) % self.options.len();
        self.current_observable
            .set(self.options[self.current_index].clone());
    }
}

impl<T> Updatable<T> for Cycle<T> {
    fn subscribe_to_updates(&self, cb: Box<dyn Fn(&T)>) {
        cb(&self.current_observable.get());
        self.current_observable.subscribe(cb);
    }
}
