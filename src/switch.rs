use esp_idf_hal::gpio::{Input, InputPin, PinDriver};
use esp_idf_sys::EspError;

pub struct Switch<'d, T: InputPin> {
    driver: PinDriver<'d, T, Input>,
    current_value: bool,
    last_value: bool,
}

impl<'d, T: InputPin> Switch<'d, T> {
    #[allow(dead_code)]
    pub fn new(pin: T) -> Result<Self, EspError> {
        let driver = PinDriver::input(pin)?;

        let value = driver.is_low();

        Ok(Self {
            driver,
            last_value: value,
            current_value: value,
        })
    }

    #[allow(dead_code)]
    pub fn poll_value(&mut self) {
        self.current_value = self.driver.is_low();
    }

    #[allow(dead_code)]
    pub fn get_value_changed(&self, new_value: bool) -> Option<bool> {
        if self.current_value != self.last_value && self.current_value == new_value {
            Some(self.current_value)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn commit_value(&mut self) {
        self.last_value = self.current_value;
    }
}
