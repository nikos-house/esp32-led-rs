use crate::color::LerpableColor;
use std::time::Instant;

#[allow(dead_code)]
pub fn stretch<T: LerpableColor + Clone>(layer: Vec<T>, length: usize) -> Vec<T> {
    let n_layer = layer.len();
    if n_layer == 0 || length == 0 {
        return Vec::new();
    }

    if n_layer == 1 {
        return vec![layer[0].clone(); length];
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

        let low_color = &layer[low_idx];
        let high_color = &layer[high_idx];

        result.push(low_color.lerp(high_color, weight));
    }

    result
}

#[allow(dead_code)]
pub fn shift<T: LerpableColor + Clone>(layer: Vec<T>, offset: f64) -> Vec<T> {
    let length = layer.len();
    if length == 0 {
        return Vec::new();
    }

    if offset == 0.0 {
        return layer;
    }

    let interpolation_factor = 1;
    let interpolated_layer = stretch(layer, length * interpolation_factor);

    let interpolated_offset = (offset * interpolation_factor as f64).round() as usize;
    let mut result = Vec::with_capacity(length);

    for i in 0..length {
        let idx =
            (i * interpolation_factor + interpolated_offset) % (length * interpolation_factor);
        result.push(interpolated_layer[idx].clone());
    }

    result
}
