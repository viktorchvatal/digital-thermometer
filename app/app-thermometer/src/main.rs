#![no_std]
#![no_main]

mod panic;
mod format;
mod sensors;
mod log;
mod display;

use core::{cell::Cell};
use arrayvec::ArrayString;
use cortex_m_rt::{entry};
use cortex_m::peripheral::Peripherals as CortexPeripherals;
use dht11::Dht11;
use display::render_display;
use embedded_hal::spi;
use embedded_sdmmc::{Controller, SdMmcSpi, TimeSource, Timestamp};
use panic::halt_with_error_led;
use hx1230::{ArrayDisplayBuffer, SpiDriver, DisplayBuffer};
use lib_datalogger::detect_sd_card_size;
use sensors::read_sensors;
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
    mut cp: cortex_m::Peripherals,
) -> Result<(), ()> {
    cp.DCB.enable_trace();
    cp.DWT.enable_cycle_counter();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(100.MHz()).hclk(25.MHz()).freeze();

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

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

    let i2c = I2c::new(
        dp.I2C1,
        (
            gpiob.pb8.into_alternate().set_open_drain(),
            gpiob.pb9.into_alternate().set_open_drain(),
        ),
        400.kHz(),
        &clocks,
    );

    let thermo_1_pin = gpiob.pb10.into_open_drain_output();
    let i2c_container = Cell::new(Some(i2c));
    let sd_cs = gpiob.pb0.into_push_pull_output();
    let mut delay = dp.TIM5.delay_us(&clocks);
    let mut frame_buffer: ArrayDisplayBuffer = ArrayDisplayBuffer::new();

    let mut display = SpiDriver::new(&mut display_spi, &mut display_cs);
    display.initialize(&mut delay).map_err(|_| ())?;

    let mut sd_controller = Controller::new(SdMmcSpi::new(sd_spi, sd_cs), Clock);
    let card_size = detect_sd_card_size(&mut sd_controller);

    let mut thermo_1_driver = Dht11::new(thermo_1_pin);

    let mut sd_result = ArrayString::<20>::new();
    print_card_size(&mut sd_result, card_size);

    loop {
        frame_buffer.clear_buffer(0x00);

        let i2c_local = Cell::new(None);
        i2c_container.swap(&i2c_local);

        let (sensors, i2c_returned) = read_sensors(
            i2c_local.into_inner().unwrap(),
            &mut thermo_1_driver,
            &mut delay
        );

        let i2c_returned_cell = Cell::new(Some(i2c_returned));
        i2c_returned_cell.swap(&i2c_container);

        render_display(&mut frame_buffer, &mut display, &sd_result, &sensors);

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