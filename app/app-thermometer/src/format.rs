use core::fmt::Write;
use embedded_sdmmc::SdMmcError;
use lib_datalogger::DatalogError;
use pcf8563::DateTime;

pub fn print_card_size(
    debug: &mut dyn Write,
    card_size: Result<u64, DatalogError<SdMmcError>>
) {
    match card_size {
        Ok(size) => {
            let _ = write!(debug, "SD Card {} MB", size >> 20);
        },
        Err(error) => {
            let _ = write!(debug, "SD Card not detected\n{}0", error);
        }
    }
}

pub fn format_date(
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