use crate::numbers::normalize;
use bno055::mint::{Quaternion, Vector3};
use bno055::{BNO055OperationMode, Bno055};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::i2c::{I2cDriver, I2cError};

pub struct Gyro<'d> {
    driver: Bno055<I2cDriver<'d>>,
}

impl<'d> Gyro<'d> {
    #[allow(dead_code)]
    pub fn new(i2c: I2cDriver<'d>) -> Result<Self, bno055::Error<I2cError>> {
        let mut imu = Bno055::new(i2c).with_alternative_address();
        imu.init(&mut FreeRtos {})?;
        imu.set_mode(BNO055OperationMode::NDOF, &mut FreeRtos {})?;

        // while !imu.is_fully_calibrated()? {
        //     println!("Calibration: {:?}", imu.get_calibration_status()?);
        // }

        Ok(Self { driver: imu })
    }

    #[allow(dead_code)]
    pub fn get_value(&mut self) -> Result<Quaternion<f32>, bno055::Error<I2cError>> {
        let value = self.driver.quaternion()?;

        Ok(value)
    }

    #[allow(dead_code)]
    pub fn get_angle(&mut self) -> Result<f32, bno055::Error<I2cError>> {
        let s = self.driver.quaternion()?.s;
        let angle = 2.0 * s.acos() * 180.0 / std::f32::consts::PI;

        Ok(angle)
    }

    #[allow(dead_code)]
    pub fn get_axis(&mut self) -> Result<Vector3<f32>, bno055::Error<I2cError>> {
        let v = self.driver.quaternion()?.v;

        let axis = normalize(&v);

        Ok(axis)
    }
}
