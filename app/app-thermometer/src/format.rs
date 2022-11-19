use core::fmt::Write;
use embedded_sdmmc::SdMmcError;
use lib_datalogger::DatalogError;

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