#![no_std]

mod error;
mod writer;
mod reader;

pub use error::DatalogError;
pub use reader::detect_sd_card_size;
pub use writer::append_to_file;