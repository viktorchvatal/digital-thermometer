#![no_std]
#![no_main]

mod panic;
mod format;

use core::fmt::Write;
use arrayvec::ArrayString;
use cortex_m_rt::{entry};
use cortex_m::peripheral::Peripherals as CortexPeripherals;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    mono_font::{MonoTextStyle, ascii::{FONT_5X8}}, text::{Text}
};
use embedded_hal::spi;
use embedded_sdmmc::{Controller, SdMmcSpi, TimeSource, Timestamp};
use format::format_date;
use panic::halt_with_error_led;
use hx1230::{ArrayDisplayBuffer, SpiDriver, DisplayBuffer, DisplayDriver};
use lib_datalogger::detect_sd_card_size;
use pcf8563::PCF8563;
use stm32f4xx_hal::{prelude::*, pac::{self, Peripherals}, gpio::NoPin, i2c::I2c};

use crate::format::print_card_size;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (Peripherals::take(), CortexPeripherals::take()) {
        let _ = run(dp, cp);
    }

    halt_with_error_led();
}

fn run(
    dp: pac::Peripherals,
    _cp: cortex_m::Peripherals,
) -> Result<(), ()> {
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(100.MHz()).hclk(25.MHz()).freeze();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();

    let mut display_cs = gpiob.pb14.into_push_pull_output();

    let mut display_spi = dp.SPI2.spi(
        (gpiob.pb13, NoPin, gpiob.pb15),
        spi::MODE_0,
        4000.kHz(),
        &clocks,
    );

    let sd_spi = dp.SPI1.spi(
        (gpioa.pa5, gpioa.pa6, gpioa.pa7),
        spi::MODE_0,
        400.kHz(),
        &clocks,
    );

    let time_i2c = I2c::new(
        dp.I2C1,
        (
            gpiob.pb8.into_alternate().set_open_drain(),
            gpiob.pb9.into_alternate().set_open_drain(),
        ),
        400.kHz(),
        &clocks,
    );

    let sd_cs = gpiob.pb0.into_push_pull_output();

    let mut delay = dp.TIM5.delay_us(&clocks);

    let mut frame_buffer: ArrayDisplayBuffer = ArrayDisplayBuffer::new();

    let mut display = SpiDriver::new(&mut display_spi, &mut display_cs);
    display.initialize(&mut delay).map_err(|_| ())?;

    let mut sd_controller = Controller::new(SdMmcSpi::new(sd_spi, sd_cs), Clock);
    let card_size = detect_sd_card_size(&mut sd_controller);

    let text_style = MonoTextStyle::new(&FONT_5X8, BinaryColor::On);

    let mut debug = ArrayString::<100>::new();
    print_card_size(&mut debug, card_size);
    let _ = writeln!(&mut debug, "");

    let mut time_driver = PCF8563::new(time_i2c);

    loop {
        frame_buffer.clear_buffer(0x00);
        let mut text = ArrayString::<100>::new();
        let _ = write!(&mut text, "{}", debug);

        match time_driver.get_datetime() {
            Ok(datetime) => {
                format_date(&mut text, datetime);
            },
            Err(error) => {
                let _ = write!(&mut text, "Time? Err:\n{:?}", error);
            }
        };

        Text::new(&text, Point::new(0, 10), text_style)
            .draw(&mut frame_buffer)
            .map_err(|_| ())?;

        let mut driver = SpiDriver::new(&mut display_spi, &mut display_cs);
        driver.send_buffer(&frame_buffer).map_err(|_| ())?;

        delay.delay_ms(400_u16);
    }
}

struct Clock;

impl TimeSource for Clock {
    // Fake time source that just returns 1. 1. 1970 0:00:00
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}