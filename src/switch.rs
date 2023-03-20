use esp_idf_hal::gpio::{InputPin, PinDriver};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_sys::EspError;
use std::marker::PhantomData;

pub struct Switch<'d, MODE> {
    driver: PinDriver<'d, dyn Peripheral<P = impl InputPin>, MODE>,
}

impl<'d, MODE> Switch<'d, MODE> {
    #[inline]
    pub fn new(pin: impl Peripheral<P = impl InputPin> + 'd) -> Result<Self, EspError> {
        esp_idf_hal::into_ref!(pin);

        let driver = PinDriver::input(pin)?;

        Ok(Self { driver })
    }
}
