mod animation;
mod color;
mod cycle;
mod gyro;
mod layer;
mod numbers;
mod runtime;
mod switch;
mod time_animation;
mod ws_led_rmt_driver;

use esp_idf_sys as _; // using `binstart` feature of `esp-idf-sys`

use esp_idf_hal::peripherals::Peripherals;

use crate::animation::{Animatable, ShiftAnimation};
use crate::cycle::Cycle;
use crate::gyro::Gyro;
use crate::runtime::{Runtime, RuntimeStatus};
use crate::time_animation::{linear, TimeAnimationRunner};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::units::FromValueType;
use rgb::RGB8;
use std::time::Duration;

fn main() {
    esp_idf_sys::link_patches();

    if let Some(peripherals) = Peripherals::take() {
        let mut runtime = Runtime::new(peripherals.pins.gpio2, peripherals.rmt.channel0);
        runtime.set_status(RuntimeStatus::Healthy);

        let mut strip_driver = match ws_led_rmt_driver::LedRmtDriver::new(
            peripherals.pins.gpio0,
            peripherals.rmt.channel1,
        ) {
            Ok(driver) => driver,
            Err(_err) => panic!("could initialize led strip"),
        };

        let mut switch = match switch::Switch::new(peripherals.pins.gpio9) {
            Ok(switch) => switch,
            Err(_err) => panic!("could initialize switch"),
        };

        let strip_length = 30;

        let mut animations = Cycle::new(vec![
            ShiftAnimation::new(
                TimeAnimationRunner::new(Duration::from_secs(5), true, linear),
                layer::stretch(
                    vec![
                        RGB8::new(130, 0, 60),
                        RGB8::new(30, 0, 130),
                        RGB8::new(130, 0, 0),
                        RGB8::new(130, 0, 60),
                    ],
                    strip_length,
                ),
            ),
            ShiftAnimation::new(
                TimeAnimationRunner::new(Duration::from_secs(5), true, linear),
                layer::stretch(
                    vec![
                        RGB8::new(130, 0, 0),
                        RGB8::new(0, 0, 130),
                        RGB8::new(0, 130, 0),
                        RGB8::new(130, 0, 0),
                    ],
                    strip_length,
                ),
            ),
            ShiftAnimation::new(
                TimeAnimationRunner::new(Duration::from_secs(5), true, linear),
                layer::stretch(
                    vec![
                        RGB8::new(130, 0, 0),
                        RGB8::new(0, 0, 130),
                        RGB8::new(130, 0, 0),
                    ],
                    strip_length,
                ),
            ),
            ShiftAnimation::new(
                TimeAnimationRunner::new(Duration::from_secs(5), true, linear),
                layer::stretch(
                    vec![
                        RGB8::new(0, 100, 0),
                        RGB8::new(100, 0, 50),
                        RGB8::new(0, 100, 0),
                    ],
                    strip_length,
                ),
            ),
            ShiftAnimation::new(
                TimeAnimationRunner::new(Duration::from_secs(5), true, linear),
                layer::stretch(
                    vec![
                        RGB8::new(100, 0, 0),
                        RGB8::new(0, 20, 0),
                        RGB8::new(100, 0, 0),
                    ],
                    strip_length,
                ),
            ),
        ]);

        let config = I2cConfig::new().baudrate(100.kHz().into());
        let i2c = I2cDriver::new(
            peripherals.i2c0,
            peripherals.pins.gpio5,
            peripherals.pins.gpio6,
            &config,
        )
        .unwrap();

        match Gyro::new(i2c) {
            Ok(mut gyro) => loop {
                switch.poll_value();

                if let Some(value) = switch.get_value_changed(true) {
                    if value {
                        animations.next();
                    }
                }
                if let Err(_err) =
                    strip_driver.write_iter(animations.get_current().get_layer().iter())
                {
                    runtime.set_status(RuntimeStatus::Error);
                }

                switch.commit_value();
                match gyro.get_angle() {
                    Ok(angle) => {
                        println!("angle: {:?}", angle);
                    }
                    Err(_err) => {
                        runtime.set_status(RuntimeStatus::Error);
                    }
                }
            },
            Err(_err) => {
                runtime.set_status(RuntimeStatus::Error);
                loop {
                    FreeRtos::delay_ms(1000);
                }
            }
        };
    }
}
