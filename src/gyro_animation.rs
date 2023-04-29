use crate::animation::AnimationRunner;
use crate::gyro::Gyro;
use esp_idf_hal::i2c::I2cError;
use esp_idf_sys::EspError;

pub struct GyroAnimationRunner<'d> {
    gyro: Gyro<'d>,
}

impl<'d> GyroAnimationRunner<'d> {
    #[allow(dead_code)]
    pub fn new(gyro: Gyro<'d>) -> Self {
        Self { gyro }
    }

    #[allow(dead_code)]
    pub fn get_progress(&mut self) -> Result<f32, bno055::Error<I2cError>> {
        Ok(self.gyro.get_angle()? / 360.0)
    }
}
