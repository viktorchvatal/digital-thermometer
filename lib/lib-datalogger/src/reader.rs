use embedded_hal::{spi::FullDuplex, digital::v2::OutputPin};
use embedded_sdmmc::{Controller, SdMmcSpi, TimeSource, SdMmcError};
use crate::error::DatalogError;

pub fn detect_sd_card_size<SPI, CS, T>(
    controller: &mut Controller<SdMmcSpi<SPI, CS>, T>,
) -> Result<u64, DatalogError<SdMmcError>>
where
    SPI: FullDuplex<u8>,
    CS: OutputPin,
    T: TimeSource,
    <SPI as FullDuplex<u8>>::Error: core::fmt::Debug
{
    match controller.device().init() {
        Ok(_) => {
            let result = controller.device()
                .card_size_bytes()
                .map_err(|error| DatalogError::CannotReadCardSize(error));

            controller.device().deinit();
            result
        }
        Err(error) => Err(DatalogError::CannotConnect(error)),
    }
}