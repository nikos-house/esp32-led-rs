mod animation;
mod color;
mod cycle;
mod layer;
mod numbers;
mod runtime;
mod switch;
mod time_animation;
mod updatable;
mod ws_led_rmt_driver;

use esp_idf_sys as _; // using `binstart` feature of `esp-idf-sys`

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use smart_leds_trait::RGB8;

use crate::animation::{Animatable, ShiftAnimation};
use crate::cycle::Cycle;
use crate::runtime::{Runtime, RuntimeStatus};
use crate::time_animation::{ease_in, ease_in_out, linear, TimeAnimationRunner};
use crate::updatable::{Pollable, Updatable};
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};

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

        let switch = match switch::Switch::new(peripherals.pins.gpio4) {
            Ok(switch) => switch,
            Err(_err) => panic!("could initialize switch"),
        };

        // let colors = Cycle::new(vec![
        //     RGBLayer::interpolate_colors(
        //         vec![
        //             RGB8::new(10, 0, 0),
        //             RGB8::new(0, 0, 10),
        //             RGB8::new(0, 10, 0),
        //         ],
        //         30,
        //     ),
        //     RGBLayer::interpolate_colors(
        //         vec![RGB8::new(0, 0, 0), RGB8::new(0, 0, 10), RGB8::new(0, 0, 0)],
        //         30,
        //     ),
        //     RGBLayer::interpolate_colors(vec![RGB8::new(0, 10, 0), RGB8::new(0, 0, 10)], 30),
        //     RGBLayer::interpolate_colors(
        //         vec![
        //             RGB8::new(1, 0, 0),
        //             RGB8::new(20, 0, 10),
        //             RGB8::new(30, 20, 0),
        //             RGB8::new(0, 20, 30),
        //             RGB8::new(0, 0, 1),
        //         ],
        //         30,
        //     ),
        //     RGBLayer::interpolate_colors(vec![RGB8::new(10, 0, 0), RGB8::new(0, 0, 10)], 30),
        //     RGBLayer::interpolate_colors(
        //         vec![
        //             RGB8::new(10, 0, 0),
        //             RGB8::new(0, 0, 10),
        //             RGB8::new(0, 10, 0),
        //             RGB8::new(10, 0, 0),
        //         ],
        //         30,
        //     ),
        // ]);
        //
        // let durable_strip_driver = rc!(strip_driver);
        // let durable_runtime = rc!(runtime);
        // colors.subscribe_to_updates(Box::new(move |layer| {
        //     let mut strip_driver_ptr = durable_strip_driver.borrow_mut();
        //     let mut runtime_ptr = durable_runtime.borrow_mut();
        //     if let Err(_err) = strip_driver_ptr.write_iter(layer.iter()) {
        //         runtime_ptr.set_status(RuntimeStatus::Error);
        //     }
        // }));
        //
        // let durable_colors = rc!(colors);
        // switch.subscribe_to_updates(Box::new(move |value| {
        //     if *value {
        //         let mut colors_ptr = durable_colors.borrow_mut();
        //         colors_ptr.next();
        //     }
        // }));

        let mut loop_colors = layer::stretch(
            vec![
                RGB8::new(40, 0, 0),
                RGB8::new(0, 0, 40),
                RGB8::new(0, 40, 0),
                RGB8::new(40, 0, 0),
            ],
            30,
        );
        let mut linear_looping_runner =
            TimeAnimationRunner::new(Duration::from_millis(2000), true, linear);
        let mut linear_looping_rainbow = ShiftAnimation::new(linear_looping_runner, loop_colors);

        loop {
            // switch.poll();
            if let Err(_err) = strip_driver.write_iter(linear_looping_rainbow.get_layer().iter()) {
                runtime.set_status(RuntimeStatus::Error);
            }
        }
    }
}
