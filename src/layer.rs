use crate::color::lerp_rgb;
use num::integer::lcm;
use num::{Integer, Signed};
use smart_leds::RGB8;
use std::cmp::min;

pub fn stretch(layer: Vec<RGB8>, length: usize) -> Vec<RGB8> {
    let n_layer = layer.len();
    if n_layer == 0 || length == 0 {
        return Vec::new();
    }

    if n_layer == 1 {
        return vec![layer[0]; length];
    }

    if n_layer == length {
        return layer;
    }

    let mut result = Vec::with_capacity(length);
    let input_step = (n_layer - 1) as f64 / (length - 1) as f64;

    for i in 0..length {
        let idx = input_step * i as f64;
        let low_idx = idx.floor() as usize;
        let high_idx = (low_idx + 1).min(n_layer - 1);

        let weight = idx.fract();

        let low_color = layer[low_idx];
        let high_color = layer[high_idx];

        result.push(lerp_rgb(low_color, high_color, weight));
    }

    result
}

pub fn shift(layer: Vec<RGB8>, offset: f64) -> Vec<RGB8> {
    let length = layer.len();
    if length == 0 {
        return Vec::new();
    }

    if offset == 0.0 {
        return layer;
    }

    let interpolation_factor = 10;
    let interpolated_layer = stretch(layer.clone(), length * interpolation_factor);

    let interpolated_offset = (offset * interpolation_factor as f64).round() as usize;
    let mut result = Vec::with_capacity(length);

    for i in 0..length {
        let idx =
            (i * interpolation_factor + interpolated_offset) % (length * interpolation_factor);
        result.push(interpolated_layer[idx]);
    }

    result
}
