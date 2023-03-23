use crate::updatable::{Pollable, Updatable};
use esp_idf_hal::gpio::{Input, InputPin, PinDriver};
use esp_idf_sys::EspError;
use observable_rs::Observable;

pub struct Switch<'d, T: InputPin> {
    driver: PinDriver<'d, T, Input>,
    value_observable: Observable<bool>,
}

impl<'d, T: InputPin> Switch<'d, T> {
    #[inline]
    pub fn new(pin: T) -> Result<Self, EspError> {
        let driver = PinDriver::input(pin)?;

        let value = driver.is_low();
        let value_observable = Observable::new(value);

        Ok(Self {
            driver,
            value_observable,
        })
    }
}

impl<'d, T: InputPin> Updatable<bool> for Switch<'d, T> {
    fn subscribe_to_updates(&self, cb: Box<dyn Fn(&bool)>) {
        self.value_observable.subscribe(cb);
    }
}
impl<'d, T: InputPin> Pollable<bool> for Switch<'d, T> {
    fn poll(&self) {
        let value = self.driver.is_low();
        if *self.value_observable.get() != value {
            self.value_observable.set(value);
        }
    }
}
