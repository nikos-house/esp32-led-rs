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
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::units::FromValueType;
use esp_idf_svc::nvs::{EspNvs, EspNvsPartition, NvsCustom, NvsDefault};
use smart_leds::{SmartLedsWrite, White};
use std::time::{Duration, Instant};
use ws2812_esp32_rmt_driver::driver::color::LedPixelColorGrbw32;
use ws2812_esp32_rmt_driver::{LedPixelEsp32Rmt, RGBW8};

fn main() {
    esp_idf_sys::link_patches();

    if let Some(peripherals) = Peripherals::take() {
        let mut runtime = Runtime::new(2, 0);
        runtime.set_status(RuntimeStatus::Healthy);

        let mut strip_driver = match LedPixelEsp32Rmt::<RGBW8, LedPixelColorGrbw32>::new(1, 0) {
            Ok(driver) => driver,
            Err(_err) => panic!("could initialize led strip"),
        };

        let mut switch = match switch::Switch::new(peripherals.pins.gpio9) {
            Ok(switch) => switch,
            Err(_err) => panic!("could initialize switch"),
        };

        let strip_length = 144;

        let mut animation_runner =
            TimeAnimationRunner::new(Duration::from_millis(1000), true, linear);
        let rainbow_animation = ShiftAnimation::new(layer::stretch(
            vec![
                RGBW8::from((40, 0, 0, White(0))),
                RGBW8::from((30, 0, 20, White(0))),
                RGBW8::from((40, 0, 0, White(0))),
            ],
            strip_length,
        ));
        let tracer_animation = ShiftAnimation::new(layer::stretch(
            vec![
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((20, 0, 0, White(0))),
                RGBW8::from((10, 0, 5, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
            ],
            strip_length,
        ));
        let double_tracer_animation = ShiftAnimation::new(layer::stretch(
            vec![
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((20, 0, 0, White(0))),
                RGBW8::from((10, 0, 5, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((20, 0, 0, White(0))),
                RGBW8::from((10, 0, 5, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
            ],
            strip_length,
        ));
        let white_animation = ShiftAnimation::new(layer::stretch(
            vec![
                RGBW8::from((0, 0, 0, White(15))),
                RGBW8::from((0, 0, 0, White(15))),
            ],
            strip_length,
        ));
        let off_animation = ShiftAnimation::new(layer::stretch(
            vec![
                RGBW8::from((0, 0, 0, White(0))),
                RGBW8::from((0, 0, 0, White(0))),
            ],
            strip_length,
        ));

        let partition = match EspNvsPartition::<NvsDefault>::take() {
            Ok(partition) => partition,
            Err(_err) => panic!("could not take nvs partition"),
        };
        let nvs = match EspNvs::new(partition, &"cycle", true) {
            Ok(nvs) => nvs,
            Err(_err) => panic!("could not initialize nvs"),
        };

        let mut animations = Cycle::new(
            &nvs,
            vec![
                rainbow_animation,
                tracer_animation,
                double_tracer_animation,
                white_animation,
                off_animation,
            ],
            &"animation",
        );

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
