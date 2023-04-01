use bno055::mint::Vector3;

// Linear interpolation
// t = 0.0 -> start
// t = 1.0 -> end
#[allow(dead_code)]
pub fn lerp(a: u8, b: u8, t: f64) -> u8 {
    (a as f64 * (1.0 - t) + b as f64 * t).round() as u8
}

#[allow(dead_code)]
pub fn normalize(v: &Vector3<f32>) -> Vector3<f32> {
    let mag = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    Vector3::from([v.x / mag, v.y / mag, v.z / mag])
}
