use crate::numbers::{angle_between_vectors, normalize, rotate_vector_by_quaternion};
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
        let q = self.driver.quaternion()?;

        // The initial direction of the sensor when it's facing down
        let init_down = Vector3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        };

        // Rotate the initial direction by the quaternion
        let rotated_down = rotate_vector_by_quaternion(&init_down, &q);

        // Calculate the angle between the rotated direction and the initial direction
        let angle = angle_between_vectors(&init_down, &rotated_down).to_degrees();

        // Check if the sensor is rotated toward the right (+Z) or the left (-Z) of the wheel
        if rotated_down.z > 0.0 {
            // If rotated to the right, return the angle directly
            Ok(angle)
        } else {
            // If rotated to the left, return the angle subtracted from 360
            Ok(360.0 - angle)
        }
    }

    #[allow(dead_code)]
    pub fn get_axis(&mut self) -> Result<Vector3<f32>, bno055::Error<I2cError>> {
        let v = self.driver.quaternion()?.v;

        let axis = normalize(&v);

        Ok(axis)
    }
}
