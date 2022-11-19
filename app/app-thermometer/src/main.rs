#![no_std]
#![no_main]

mod panic;
mod format;

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
use panic::halt_with_error_led;
use hx1230::{ArrayDisplayBuffer, SpiDriver, DisplayBuffer, DisplayDriver};
use lib_datalogger::detect_sd_card_size;
use stm32f4xx_hal::{prelude::*, pac::{self, Peripherals}, gpio::NoPin};

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

    let mut spi = dp.SPI2.spi(
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

    let sd_cs = gpiob.pb0.into_push_pull_output();

    let mut delay = dp.TIM5.delay_us(&clocks);

    let mut frame_buffer: ArrayDisplayBuffer = ArrayDisplayBuffer::new();

    let mut display = SpiDriver::new(&mut spi, &mut display_cs);
    display.initialize(&mut delay).map_err(|_| ())?;

    let mut sd_controller = Controller::new(SdMmcSpi::new(sd_spi, sd_cs), Clock);
    let card_size = detect_sd_card_size(&mut sd_controller);

    let text_style = MonoTextStyle::new(&FONT_5X8, BinaryColor::On);

    let mut debug = ArrayString::<200>::new();
    print_card_size(&mut debug, card_size);

    loop {
        frame_buffer.clear_buffer(0x00);

        Text::new(&debug, Point::new(0, 10), text_style)
            .draw(&mut frame_buffer)
            .map_err(|_| ())?;

        let mut driver = SpiDriver::new(&mut spi, &mut display_cs);
        driver.send_buffer(&frame_buffer).map_err(|_| ())?;

        delay.delay_ms(1000_u16);
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