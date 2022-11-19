use bmp280_rs::{BMP280, I2CAddress};
use dht11::{Dht11, Measurement};
use pcf8563::{PCF8563, DateTime};
use embedded_hal::blocking::{i2c, delay::{DelayUs, DelayMs}};
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub struct Sensors {
    pub time: Option<DateTime>,
    pub temperature_pressure: Option<TemperaturePressure>,
    pub temperature_humidity: [Option<Measurement>; 4],
}

pub fn read_sensors<I2C, I2CE, D, T1, TE>(
    i2c: I2C,
    thermo_1_driver: &mut Dht11<T1>,
    delay: &mut D
) -> (Sensors, I2C)
where
    I2C: i2c::Write<Error = I2CE> + i2c::WriteRead<Error = I2CE>,
    I2CE: core::fmt::Debug,
    D: DelayUs<u16> + DelayMs<u16>,
    T1: InputPin<Error = TE> + OutputPin<Error = TE>,
{
    let mut time_driver = PCF8563::new(i2c);
    let time = time_driver.get_datetime().ok();
    let mut i2c = time_driver.destroy();
    let temperature_pressure = read_bmp280(&mut i2c);
    let temperature_humidity_1 = read_dht11(thermo_1_driver, delay);

    let sensors = Sensors {
        time,
        temperature_pressure,
        temperature_humidity: [
            temperature_humidity_1,
            None,
            None,
            None,
        ]
    };

    (sensors, i2c)
}

pub struct TemperaturePressure {
    /// In 1/100 degrees celsius
    pub temperature: i32,
    /// In pascals
    pub pressure: i32,
}

fn read_dht11<T, D, E>(
    driver: &mut Dht11<T>,
    delay: &mut D
) -> Option<Measurement>
where
    D: DelayUs<u16> + DelayMs<u16>,
    T: InputPin<Error = E> + OutputPin<Error = E>
{
    driver.perform_measurement(delay).ok()
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
