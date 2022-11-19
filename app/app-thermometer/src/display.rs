use core::fmt::Write;
use arrayvec::ArrayString;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    mono_font::{MonoTextStyle, ascii::{FONT_5X8}}, text::{Text}
};
use crate::format::{format_sensors_display};
use hx1230::{ArrayDisplayBuffer, DisplayBuffer, DisplayDriver};

use crate::sensors::Sensors;

pub fn render_display(
    buffer: &mut ArrayDisplayBuffer,
    driver: &mut dyn DisplayDriver,
    sd_result: &str,
    sensors: &Sensors
) {
    buffer.clear_buffer(0x00);
    let mut text = ArrayString::<200>::new();
    let _ = writeln!(&mut text, "{}", sd_result);
    format_sensors_display(&mut text, &sensors);
    let style = MonoTextStyle::new(&FONT_5X8, BinaryColor::On);
    let position = Point::new(0, 8);
    let _ = Text::new(&text, position, style).draw(buffer);
    let _ = driver.send_buffer(buffer);
}