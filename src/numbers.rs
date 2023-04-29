use bno055::mint::{Quaternion, Vector3};

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

#[allow(dead_code)]
pub fn rotate_vector_by_quaternion(v: &Vector3<f32>, q: &Quaternion<f32>) -> Vector3<f32> {
    let q_conjugate = Quaternion {
        s: q.s,
        v: Vector3 {
            x: -q.v.x,
            y: -q.v.y,
            z: -q.v.z,
        },
    };
    let p = Quaternion { s: 0.0, v: *v };
    let result = quaternion_mul(&quaternion_mul(q, &p), &q_conjugate);
    result.v
}

fn quaternion_mul(a: &Quaternion<f32>, b: &Quaternion<f32>) -> Quaternion<f32> {
    let s = a.s * b.s - a.v.x * b.v.x - a.v.y * b.v.y - a.v.z * b.v.z;
    let v = Vector3 {
        x: a.s * b.v.x + a.v.x * b.s + a.v.y * b.v.z - a.v.z * b.v.y,
        y: a.s * b.v.y - a.v.x * b.v.z + a.v.y * b.s + a.v.z * b.v.x,
        z: a.s * b.v.z + a.v.x * b.v.y - a.v.y * b.v.x + a.v.z * b.s,
    };
    Quaternion { s, v }
}

#[allow(dead_code)]
pub fn angle_between_vectors(v1: &Vector3<f32>, v2: &Vector3<f32>) -> f32 {
    let dot_product = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
    let magnitude_v1 = (v1.x * v1.x + v1.y * v1.y + v1.z * v1.z).sqrt();
    let magnitude_v2 = (v2.x * v2.x + v2.y * v2.y + v2.z * v2.z).sqrt();
    (dot_product / (magnitude_v1 * magnitude_v2)).acos()
}
