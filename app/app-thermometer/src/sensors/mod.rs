mod temperature_dht11;

use core::fmt::Display;

use bmp280_rs::{BMP280, I2CAddress};
use dht11::{Measurement};
use pcf8563::{PCF8563, DateTime};
use embedded_hal::blocking::{i2c, delay::{DelayUs, DelayMs}};

pub use temperature_dht11::{Dht11Drivers, Dht11Reader};

pub struct Sensors {
    pub time: Option<DateTime>,
    pub temperature_pressure: Option<TemperaturePressure>,
    pub temperature_humidity: [Option<Measurement>; 6],
}

impl Sensors {
    pub fn get_time(&self) -> Option<Time> {
        self.time.map(|date_time| Time {
            hours: date_time.hours,
            minutes: date_time.minutes,
            seconds: date_time.seconds,
        })
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
    }
}

pub fn read_sensors<I2C, I2CE, D>(
    i2c: I2C,
    thermo_drivers: &mut dyn Dht11Reader<D>,
    delay: &mut D
) -> (Sensors, I2C)
where
    I2C: i2c::Write<Error = I2CE> + i2c::WriteRead<Error = I2CE>,
    I2CE: core::fmt::Debug,
    D: DelayUs<u16> + DelayMs<u16>,
{
    let mut time_driver = PCF8563::new(i2c);
    let time = time_driver.get_datetime().ok();
    let mut i2c = time_driver.destroy();
    let temperature_pressure = read_bmp280(&mut i2c);

    let temperature_humidity = thermo_drivers.read(delay);

    let sensors = Sensors {
        time,
        temperature_pressure,
        temperature_humidity
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
