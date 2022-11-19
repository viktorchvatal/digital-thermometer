use core::fmt::Debug;

use bmp280_rs::{BMP280, I2CAddress};
use pcf8563::{PCF8563, DateTime};
use embedded_hal::blocking::i2c;

pub struct Sensors {
    pub time: Option<DateTime>,
    pub temperature_pressure: Option<TemperaturePressure>,
}

pub fn read_sensors<I2C, I2CE>(i2c: I2C) -> (Sensors, I2C)
where
    I2C: i2c::Write<Error = I2CE> + i2c::WriteRead<Error = I2CE>,
    I2CE: core::fmt::Debug
{
    let mut time_driver = PCF8563::new(i2c);
    let time = time_driver.get_datetime().ok();
    let mut i2c = time_driver.destroy();
    let temperature_pressure = read_bmp280(&mut i2c);

    let sensors = Sensors {
        time,
        temperature_pressure
    };

    (sensors, i2c)
}

pub struct TemperaturePressure {
    /// In 1/100 degrees celsius
    pub temperature: i32,
    /// In pascals
    pub pressure: i32,
}

fn read_bmp280<I2C, I2CE>(i2c: &mut I2C) -> Option<TemperaturePressure>
where
    I2C: i2c::Write<Error = I2CE> + i2c::WriteRead<Error = I2CE>,
    I2CE: core::fmt::Debug
{
    let config = bmp280_rs::Config::handheld_device_dynamic();

    match BMP280::new(i2c, I2CAddress::SdoGrounded, config) {
        Ok(mut driver) => {
            driver.trigger_measurement(i2c).ok()?;
            let raw_pressure = driver.read_pressure(i2c).ok()?;
            let temperature = driver.read_temperature(i2c).ok()?;

            let values = TemperaturePressure {
                temperature,
                pressure: raw_pressure/256,
            };

            Some(values)
        },
        Err(_error) => None,
    }
}
