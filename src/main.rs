mod switch;
mod ws_led_rmt_driver;

use esp_idf_sys as _; // using `binstart` feature of `esp-idf-sys`

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use smart_leds_trait::RGB8;

fn main() {
    esp_idf_sys::link_patches();

    if let Some(peripherals) = Peripherals::take() {
        let mut strip_driver = ws_led_rmt_driver::LedRmtDriver::new(
            peripherals.pins.gpio21,
            peripherals.rmt.channel0,
        )?;
        let mut status_driver =
            ws_led_rmt_driver::LedRmtDriver::new(peripherals.pins.gpio2, peripherals.rmt.channel1)?;

        match status_driver.write(RGB8::new(0, 1, 0)) {
            Err(_err) => panic!("could not write status led"),
            _ => {}
        };

        loop {
            match strip_driver.write_n(RGB8::new(1, 0, 0), 300) {
                Err(_err) => match status_driver.write(RGB8::new(1, 0, 0)) {
                    Err(_err) => panic!("could not write status led"),
                    _ => {}
                },
                _ => {}
            };

            FreeRtos::delay_ms(1000);
        }
    }
}
