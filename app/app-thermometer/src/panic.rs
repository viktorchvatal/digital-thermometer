use core::panic::PanicInfo;
use stm32f4xx_hal::{pac, prelude::*};

/// Turn on error LED and halt
pub fn halt_with_error_led() -> ! {
    // We are the last to use these peripherals, so it is OK to steal them
    let dp = unsafe { pac::Peripherals::steal() };
    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();
    let _ = led.set_low();
    loop { }
}

/// Turn on onboard LED in case of panic
#[inline(never)]
#[panic_handler]
pub fn on_panic(_info: &PanicInfo) -> ! {
    halt_with_error_led();
}