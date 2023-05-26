mod animation;
mod color;
mod cycle;
mod gyro;
mod gyro_animation;
mod layer;
mod numbers;
mod runtime;
mod switch;
mod time_animation;

use esp_idf_sys as _;
use std::ops::Deref; // using `binstart` feature of `esp-idf-sys`

use esp_idf_hal::peripherals::Peripherals;

use crate::animation::{Animatable, AnimationRunner, ShiftAnimation};
use crate::cycle::Cycle;
use crate::gyro::Gyro;
use crate::gyro_animation::GyroAnimationRunner;
use crate::layer::shift;
use crate::runtime::{Runtime, RuntimeStatus};
use crate::time_animation::{linear, TimeAnimationRunner};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::Pin;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::units::FromValueType;
use esp_idf_svc::nvs::{EspNvs, EspNvsPartition, NvsCustom, NvsDefault};
use smart_leds::{SmartLedsWrite, White, RGB8};
use std::time::{Duration, Instant};
use ws2812_esp32_rmt_driver::driver::color::{LedPixelColorGrb24, LedPixelColorGrbw32};
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, RGBW8};

fn main() {
    esp_idf_sys::link_patches();

    if let Some(peripherals) = Peripherals::take() {
        let mut runtime = Runtime::new(peripherals.pins.gpio2.pin() as u32, 0);
        runtime.set_status(RuntimeStatus::Healthy);

        let mut strip_driver = match LedPixelEsp32Rmt::<RGB8, LedPixelColorGrb24>::new(
            1,
            peripherals.pins.gpio5.pin() as u32,
        ) {
            Ok(driver) => driver,
            Err(_err) => panic!("could initialize led strip"),
        };

        let mut switch = match switch::Switch::new(peripherals.pins.gpio20) {
            Ok(switch) => switch,
            Err(_err) => panic!("could initialize switch"),
        };

        let strip_length = 450;

        let mut animation_runner =
            TimeAnimationRunner::new(Duration::from_millis(1000), true, linear);
        let rainbow_animation = ShiftAnimation::new(layer::stretch(
            vec![
                RGB8::new(20, 20, 20),
                RGB8::new(20, 20, 20),
                RGB8::new(20, 20, 20),
            ],
            strip_length,
        ));

        let partition = match EspNvsPartition::<NvsDefault>::take() {
            Ok(partition) => partition,
            Err(_err) => panic!("could not take nvs partition"),
        };
        let nvs = match EspNvs::new(partition, &"cycle2", true) {
            Ok(nvs) => nvs,
            Err(_err) => panic!("could not initialize nvs"),
        };

        let mut animations = Cycle::new(&nvs, vec![rainbow_animation], &"animation");

        loop {
            switch.poll_value();

            if let Some(_) = switch.get_value_changed(true) {
                animations.next();
            }
            let progress = animation_runner.get_progress();

            let animation = animations.get_current();
            let new_layer = animation.get_layer(progress);

            if let Err(_err) = strip_driver.write(new_layer.into_iter()) {
                runtime.set_status(RuntimeStatus::Error);
            }

            switch.commit_value();
        }
    }
}
