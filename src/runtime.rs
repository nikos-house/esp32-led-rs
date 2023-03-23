use crate::ws_led_rmt_driver::LedRmtDriver;
use esp_idf_hal::gpio::OutputPin;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::rmt::RmtChannel;
use smart_leds_trait::RGB8;

pub enum RuntimeStatus {
    Healthy,
    Error,
}

pub struct Runtime<'d> {
    status_driver: LedRmtDriver<'d>,
}

impl<'d> Runtime<'d> {
    pub fn new(
        status_pin: impl Peripheral<P = impl OutputPin> + 'd,
        status_channel: impl Peripheral<P = impl RmtChannel> + 'd,
    ) -> Self {
        let status_driver = match LedRmtDriver::new(status_pin, status_channel) {
            Ok(driver) => driver,
            Err(_err) => panic!("could initialize status led"),
        };

        Self { status_driver }
    }

    pub fn set_status(&mut self, status: RuntimeStatus) {
        match status {
            RuntimeStatus::Healthy => {
                if let Err(_err) = self.status_driver.write(&RGB8::new(0, 1, 0)) {
                    panic!("could not write status led")
                }
            }
            RuntimeStatus::Error => {
                if let Err(_err) = self.status_driver.write(&RGB8::new(1, 0, 0)) {
                    panic!("could not write status led")
                }
            }
        }
    }
}
