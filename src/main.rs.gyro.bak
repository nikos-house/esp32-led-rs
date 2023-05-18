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
mod ws_led_rmt_driver;

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

        let config = I2cConfig::new().baudrate(100.kHz().into());
        match I2cDriver::new(
            peripherals.i2c0,
            peripherals.pins.gpio5,
            peripherals.pins.gpio6,
            &config,
        ) {
            Ok(i2c) => {
                match Gyro::new(i2c) {
                    Ok(gyro) => {
                        let strip_length = 160;

                        let mut animation_runner = GyroAnimationRunner::new(gyro);
                        let rainbow_animation = ShiftAnimation::new(layer::stretch(
                            vec![
                                RGB8::new(10, 0, 0),
                                RGB8::new(5, 0, 10),
                                RGB8::new(10, 0, 0),
                            ],
                            strip_length,
                        ));

                        let red_pointer = ShiftAnimation::new(layer::stretch(
                            vec![
                                RGB8::new(10, 0, 0),
                                RGB8::new(0, 0, 0),
                                RGB8::new(0, 0, 0),
                                RGB8::new(0, 0, 0),
                                RGB8::new(0, 0, 0),
                                RGB8::new(0, 0, 0),
                                RGB8::new(0, 0, 0),
                                RGB8::new(0, 0, 0),
                                RGB8::new(0, 0, 0),
                                RGB8::new(0, 0, 0),
                                RGB8::new(0, 0, 0),
                                RGB8::new(10, 0, 0),
                            ],
                            strip_length,
                        ));

                        let mut animations = Cycle::new(vec![rainbow_animation, red_pointer]);

                        loop {
                            switch.poll_value();

                            if let Some(value) = switch.get_value_changed(true) {
                                if value {
                                    animations.next();
                                }
                            }
                            match animation_runner.get_progress() {
                                Ok(progress) => {
                                    let new_layer = animations.get_current().get_layer(progress);
                                    if let Err(_err) = strip_driver.write_iter(new_layer.iter()) {
                                        runtime.set_status(RuntimeStatus::Error);
                                    }
                                }
                                Err(_err) => {
                                    runtime.set_status(RuntimeStatus::Error);
                                }
                            }

                            switch.commit_value();
                        }
                    }
                    Err(_err) => {
                        runtime.set_status(RuntimeStatus::Error);
                        loop {
                            FreeRtos::delay_ms(1000);
                        }
                    }
                };
            }
            Err(_err) => {
                runtime.set_status(RuntimeStatus::Error);
            }
        };
    }
}
