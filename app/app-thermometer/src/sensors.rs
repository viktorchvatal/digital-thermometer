use core::fmt::Display;

use bmp280_rs::{BMP280, I2CAddress};
use dht11::{Dht11, Measurement};
use pcf8563::{PCF8563, DateTime};
use embedded_hal::blocking::{i2c, delay::{DelayUs, DelayMs}};
use embedded_hal::digital::v2::{InputPin, OutputPin};

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

pub fn read_sensors<I2C, I2CE, D, T0, T1, T2, T3, T4, T5, TE>(
    i2c: I2C,
    thermo_drivers: &mut (
        Dht11<T0>,
        Dht11<T1>,
        Dht11<T2>,
        Dht11<T3>,
        Dht11<T4>,
        Dht11<T5>,
    ),
    delay: &mut D
) -> (Sensors, I2C)
where
    I2C: i2c::Write<Error = I2CE> + i2c::WriteRead<Error = I2CE>,
    I2CE: core::fmt::Debug,
    D: DelayUs<u16> + DelayMs<u16>,
    T0: InputPin<Error = TE> + OutputPin<Error = TE>,
    T1: InputPin<Error = TE> + OutputPin<Error = TE>,
    T2: InputPin<Error = TE> + OutputPin<Error = TE>,
    T3: InputPin<Error = TE> + OutputPin<Error = TE>,
    T4: InputPin<Error = TE> + OutputPin<Error = TE>,
    T5: InputPin<Error = TE> + OutputPin<Error = TE>,
{
    let mut time_driver = PCF8563::new(i2c);
    let time = time_driver.get_datetime().ok();
    let mut i2c = time_driver.destroy();
    let temperature_pressure = read_bmp280(&mut i2c);

    let temperature_humidity = [
        read_dht11(&mut thermo_drivers.0, delay),
        read_dht11(&mut thermo_drivers.1, delay),
        read_dht11(&mut thermo_drivers.2, delay),
        read_dht11(&mut thermo_drivers.3, delay),
        read_dht11(&mut thermo_drivers.4, delay),
        read_dht11(&mut thermo_drivers.5, delay),
    ];

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
