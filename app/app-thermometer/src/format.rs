use core::fmt::Write;
use dht11::Measurement;
use embedded_sdmmc::SdMmcError;
use lib_datalogger::DatalogError;
use pcf8563::DateTime;

use crate::sensors::{Sensors, TemperaturePressure};

pub fn print_card_size(
    debug: &mut dyn Write,
    card_size: Result<u64, DatalogError<SdMmcError>>
) {
    match card_size {
        Ok(size) => {
            let _ = write!(debug, "SD Card {} MB", size >> 20);
        },
        Err(error) => {
            let _ = write!(debug, "SD Card undetected\n{}", error);
        }
    }
}

pub fn format_sensors_display(
    output: &mut dyn Write,
    sensors: &Sensors,
) {
    match sensors.time {
        Some(time) => format_date(output, time),
        None => { let _ = write!(output, "Time unknown"); },
    };

    let _ = writeln!(output);

    match sensors.temperature_pressure {
        Some(ref values) => format_temperature_pressure(output, values),
        None => { let _ = write!(output, "TempPres unknown"); },
    };

    let _ = writeln!(output);

    for temperature_humidity in sensors.temperature_humidity.iter() {
        match temperature_humidity {
            Some(ref values) => format_temperature_humidity(output, values),
            None => { let _ = write!(output, "TempHumi unknown"); },
        };

        let _ = writeln!(output);
    }
}

fn format_temperature_humidity(
    output: &mut dyn Write,
    values: &Measurement
) {
    let _ = write!(output, "{}.{} C", values.temperature/10, values.temperature%10);
    let _ = write!(output, "   ");
    let _ = write!(output, "{}.{} %", values.humidity/10, values.humidity%10);
}

fn format_temperature_pressure(
    output: &mut dyn Write,
    values: &TemperaturePressure
)  {
    let _ = write!(output, "{}.{:02} C", values.temperature/100, values.temperature % 100);
    let _ = write!(output, "  ");
    let _ = write!(output, "{}.{:02} hPa", values.pressure/100, values.pressure % 100);
}

fn format_date(
    destination: &mut dyn Write,
    datetime: DateTime
)  {
    let _ = write!(
        destination,
        "{:02}.{:02}.20{:02} {:02}:{:02}:{:02}",
        datetime.day,
        datetime.month,
        datetime.year,
        datetime.hours,
        datetime.minutes,
        datetime.seconds,
    );
}