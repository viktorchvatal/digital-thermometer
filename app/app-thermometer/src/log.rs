use core::fmt::Write;
use arrayvec::ArrayString;
use dht11::Measurement;
use pcf8563::DateTime;

use crate::sensors::{Sensors, TemperaturePressure};

pub fn format_log_file(
    sensors: &Sensors,
) -> Option<ArrayString<15>> {
    sensors.time.as_ref().and_then(|time| {
        let mut buffer = ArrayString::<15>::new();

        match format_log_file_name(&mut buffer, time) {
            Ok(_) => Some(buffer),
            Err(_) => None,
        }
    })
}

pub fn format_sensors_log(
    output: &mut dyn Write,
    sensors: &Sensors,
) {
    print_optional(output, sensors.time.as_ref(), format_date);
    print_optional(output, sensors.time.as_ref(), format_time);
    print_optional(output, sensors.temperature_pressure.as_ref(), format_bmp280_temperature);
    print_optional(output, sensors.temperature_pressure.as_ref(), format_bmp280_pressure);

    for temperature_humidity in sensors.temperature_humidity.iter() {
        print_optional(output, temperature_humidity.as_ref(), format_dht11_temperature);
        print_optional(output, temperature_humidity.as_ref(), format_dht11_humidity);
    }

    let _ = write!(output, "End");
}

fn print_optional<T, F>(
    output: &mut dyn Write,
    value: Option<&T>,
    formatter: F
) where F: Fn(&mut dyn Write, &T) -> Result<(), core::fmt::Error> {
    match value {
        Some(value) => { let _ = formatter(output, value);},
        None => { let _ = write!(output, "? "); },
    }
}

fn format_log_file_name(
    output: &mut dyn Write,
    value: &DateTime,
) -> Result<(), core::fmt::Error> {
    write!(output, "20{:02}{:02}{:02}.log", value.year, value.month, value.day)
}

fn format_date(
    output: &mut dyn Write,
    value: &DateTime,
) -> Result<(), core::fmt::Error> {
    write!(output, "20{:02}-{:02}-{:02}", value.year, value.month, value.day)
}

fn format_time(
    output: &mut dyn Write,
    value: &DateTime,
) -> Result<(), core::fmt::Error> {
    write!(output, "{:02}:{:02}:{:02}", value.hours, value.minutes, value.seconds,)
}

fn format_bmp280_temperature(
    output: &mut dyn Write,
    value: &TemperaturePressure,
) -> Result<(), core::fmt::Error> {
    write!(output, "{}.{:02}", value.temperature/100, value.temperature % 100)
}

fn format_bmp280_pressure(
    output: &mut dyn Write,
    value: &TemperaturePressure,
) -> Result<(), core::fmt::Error> {
    write!(output, "{}.{:02} hPa", value.pressure/100, value.pressure % 100)
}

fn format_dht11_temperature(
    output: &mut dyn Write,
    value: &Measurement,
) -> Result<(), core::fmt::Error> {
    write!(output, "{}.{:02}", value.temperature/100, value.temperature % 100)
}

fn format_dht11_humidity(
    output: &mut dyn Write,
    value: &Measurement,
) -> Result<(), core::fmt::Error> {
    write!(output, "{}.{} %", value.humidity/10, value.humidity%10)
}