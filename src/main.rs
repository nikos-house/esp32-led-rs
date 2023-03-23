mod cycle;
mod runtime;
mod switch;
mod updatable;
mod ws_led_rmt_driver;

use esp_idf_sys as _; // using `binstart` feature of `esp-idf-sys`

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use smart_leds_trait::RGB8;

use crate::cycle::Cycle;
use crate::runtime::{Runtime, RuntimeStatus};
use crate::updatable::{Pollable, Updatable};
use std::{cell::RefCell, rc::Rc};

fn main() {
    esp_idf_sys::link_patches();

    if let Some(peripherals) = Peripherals::take() {
        let mut runtime = Runtime::new(peripherals.pins.gpio2, peripherals.rmt.channel0);
        runtime.set_status(RuntimeStatus::Healthy);

        let strip_driver = match ws_led_rmt_driver::LedRmtDriver::new(
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

        let colors = Cycle::new(vec![
            RGB8::new(0, 0, 0),
            RGB8::new(1, 0, 0),
            RGB8::new(0, 1, 0),
            RGB8::new(0, 0, 1),
            RGB8::new(1, 1, 1),
            RGB8::new(1, 1, 0),
            RGB8::new(1, 0, 1),
            RGB8::new(0, 1, 1),
        ]);

        let durable_strip_driver = rc!(strip_driver);
        let durable_runtime = rc!(runtime);
        colors.subscribe_to_updates(Box::new(move |rgb| {
            let mut strip_driver_ptr = durable_strip_driver.borrow_mut();
            let mut runtime_ptr = durable_runtime.borrow_mut();
            if let Err(_err) = strip_driver_ptr.write_n(rgb, 30) {
                runtime_ptr.set_status(RuntimeStatus::Error);
            }
        }));

        let durable_colors = rc!(colors);
        switch.subscribe_to_updates(Box::new(move |value| {
            if *value {
                let mut colors_ptr = durable_colors.borrow_mut();
                colors_ptr.next();
            }
        }));

        loop {
            switch.poll();
            FreeRtos::delay_ms(1);
        }
    }
}
