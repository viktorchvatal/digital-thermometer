use core::{fmt::{Debug, Display}};
use embedded_sdmmc::{Error, SdMmcError};

pub enum DatalogError<E>
where E: core::fmt::Debug {
    CannotConnect(SdMmcError),
    CannotReadCardSize(SdMmcError),
    NoSuitableVolume,
    CannotReadRootDir(Error<E>),
    CannotOpenFile(Error<E>),
    CannotWriteToOpenedFile(Error<E>),
}

impl<T> Display for DatalogError<T> where T: Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DatalogError::CannotConnect(ref err)
                => write!(f, "Conn:{}", device_error_to_str(err)),
            DatalogError::CannotReadCardSize(ref err)
                => write!(f, "Size:{}", device_error_to_str(err)),
            DatalogError::NoSuitableVolume
                => write!(f, "Volu"),
                DatalogError::CannotReadRootDir(ref err)
                => write!(f, "Root:{}", controller_error_to_str(err)),
            DatalogError::CannotOpenFile(ref err)
                => write!(f, "Open:{}", controller_error_to_str(err)),
            DatalogError::CannotWriteToOpenedFile(ref err)
                => write!(f, "WrEr:{}", controller_error_to_str(err)),
        }
    }
}

fn device_error_to_str(error: &SdMmcError) -> &'static str {
    match error {
        SdMmcError::Transport => "Transport",
        SdMmcError::CantEnableCRC => "EnableCrc",
        SdMmcError::TimeoutReadBuffer => "TOReadBuf",
        SdMmcError::TimeoutWaitNotBusy => "TOWaitNoBusy",
        SdMmcError::TimeoutCommand(_) => "TOCommand",
        SdMmcError::TimeoutACommand(_) => "TOACommand",
        SdMmcError::Cmd58Error => "Cmd58Err",
        SdMmcError::RegisterReadError => "RegReadErr",
        SdMmcError::CrcError(_, _) => "Crc",
        SdMmcError::ReadError => "ReadErr",
        SdMmcError::WriteError => "WriteErr",
        SdMmcError::BadState => "BadState",
        SdMmcError::CardNotFound => "CardNotFound",
        SdMmcError::GpioError => "GpioErr",
    }
}

fn controller_error_to_str<E>(error: &Error<E>)-> &'static str where E: Debug {
    match error {
        Error::DeviceError(_) => "DevErr",
        Error::FormatError(_) => "FormatErr",
        Error::NoSuchVolume => "NoVol",
        Error::FilenameError(_) => "FNameErr",
        Error::TooManyOpenDirs => "ManyOpenDirs",
        Error::TooManyOpenFiles => "ManyOpenFiles",
        Error::FileNotFound => "FileNotFound",
        Error::FileAlreadyOpen => "FAlreadyOpen",
        Error::DirAlreadyOpen => "DirAlreadyOpen",
        Error::OpenedDirAsFile => "OpenDirAsFile",
        Error::Unsupported => "Unsupported",
        Error::EndOfFile => "EOF",
        Error::BadCluster => "BadCluster",
        Error::ConversionError => "ConvertErr",
        Error::NotEnoughSpace => "NoSpace",
        Error::AllocationError => "AllocErr",
        Error::JumpedFree => "JumpedFree",
        Error::ReadOnly => "ReadOnly",
        Error::FileAlreadyExists => "FileExists",
    }
}
