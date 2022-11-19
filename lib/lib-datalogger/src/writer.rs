use core::{fmt::{Debug}};
use embedded_hal::{spi::FullDuplex, digital::v2::OutputPin};
use embedded_sdmmc::{
    Controller, SdMmcSpi, TimeSource, VolumeIdx, Volume, Mode, Directory, File, BlockDevice,
    SdMmcError
};
use crate::error::DatalogError;

/// Connect to Sd card and append the given `file_data` to the file named
/// `file_name` (file is created if not exists), on the first suitable
/// primary partition (if found) in the card root directory
pub fn append_to_file<SPI, CS, T>(
    controller: &mut Controller<SdMmcSpi<SPI, CS>, T>,
    file_name: &str,
    file_data: &str,
) -> Result<(), DatalogError<SdMmcError>>
where
    SPI: FullDuplex<u8>,
    CS: OutputPin,
    T: TimeSource,
    <SPI as FullDuplex<u8>>::Error: core::fmt::Debug
{
    match controller.device().init() {
        Ok(_) => {
            let result = write_to_volume(controller, file_name, file_data);
            controller.device().deinit();
            result
        },
        Err(error) => Err(DatalogError::CannotConnect(error)),
    }
}

fn write_to_volume<D, T, E>(
    controller: &mut Controller<D, T>,
    file_name: &str,
    file_data: &str,
) -> Result<(), DatalogError<E>>
where D: BlockDevice<Error = E>, T: TimeSource, E: Debug {
    let mut volume = open_volume(controller)?;

    match controller.open_root_dir(&mut volume) {
        Ok(dir) => {
            let result = write_to_file_in_dir(controller, &dir, &mut volume, file_name, file_data);
            controller.close_dir(&mut volume, dir);
            result
        },
        Err(error) => Err(DatalogError::CannotReadRootDir(error)),
    }
}

fn write_to_file_in_dir<D, T, E>(
    controller: &mut Controller<D, T>,
    directory: &Directory,
    volume: &mut Volume,
    file_name: &str,
    file_data: &str,
) -> Result<(), DatalogError<E>>
where D: BlockDevice<Error = E>, T: TimeSource, E: Debug {
    match controller.open_file_in_dir(
        volume, &directory, file_name, Mode::ReadWriteCreateOrAppend
    ) {
        Ok(mut file) => {
            let result = write_to_opened_file(controller, volume, &mut file, file_data);
            let _ = controller.close_file(volume, file);
            result
        }
        Err(error) => Err(DatalogError::CannotOpenFile(error)),
    }
}

fn write_to_opened_file<D, T, E>(
    controller: &mut Controller<D, T>,
    volume: &mut Volume,
    file: &mut File,
    file_data: &str,
) -> Result<(), DatalogError<E>>
where D: BlockDevice<Error = E>, T: TimeSource, E: Debug {
    match controller.write(volume, file, file_data.as_bytes()) {
        Ok(_) => Ok(()),
        Err(error) => Err(DatalogError::CannotWriteToOpenedFile(error)),
    }
}

fn open_volume<D, T, E>(
    controller: &mut Controller<D, T>,
) -> Result<Volume, DatalogError<E>>
where D: BlockDevice<Error = E>, T: TimeSource, E: Debug {
    for volume_index in 0..4 {
        if let Ok(volume) = controller.get_volume(VolumeIdx(volume_index)) {
            return Ok(volume);
        }
    }

    return  Err(DatalogError::NoSuitableVolume);
}