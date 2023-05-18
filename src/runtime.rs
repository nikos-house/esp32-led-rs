use esp_idf_hal::gpio::OutputPin;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::rmt::RmtChannel;
use rgb::RGB8;
use smart_leds::SmartLedsWrite;
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrb24;
use ws2812_esp32_rmt_driver::LedPixelEsp32Rmt;

pub enum RuntimeStatus {
    Healthy,
    Error,
}

pub struct Runtime {
    status_driver: LedPixelEsp32Rmt<RGB8, LedPixelColorGrb24>,
}

impl Runtime {
    #[allow(dead_code)]
    pub fn new(status_pin: u32, status_channel: u8) -> Self {
        let status_driver =
            match LedPixelEsp32Rmt::<RGB8, LedPixelColorGrb24>::new(status_channel, status_pin) {
                Ok(driver) => driver,
                Err(_err) => panic!("could initialize led strip"),
            };
        Self { status_driver }
    }

    #[allow(dead_code)]
    pub fn set_status(&mut self, status: RuntimeStatus) {
        match status {
            RuntimeStatus::Healthy => {
                if let Err(_err) = self
                    .status_driver
                    .write(std::iter::once(RGB8::new(0, 1, 0)))
                {
                    panic!("could not write status led")
                }
            }
            RuntimeStatus::Error => {
                if let Err(_err) = self
                    .status_driver
                    .write(std::iter::once(RGB8::new(1, 0, 0)))
                {
                    panic!("could not write status led")
                }
            }
        }
    }
}
