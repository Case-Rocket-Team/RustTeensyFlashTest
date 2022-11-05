//! The starter code slowly blinks the LED, and sets up
//! USB logging.

#![no_std]
#![no_main]

use bsp::pins::imxrt_iomuxc::spi::Pin;
use cortex_m::peripheral::syst::SystClkSource;

use cortex_m_rt::exception;
use teensy4_bsp as bsp;
use teensy4_panic as _;
use core::time::Duration;

mod logging;

const SPI_HZ: u32 = 75_000_000;
const CORTEX_HZ: u32 = 600_000_000;

// Systick will count up to this value, fire the exception and then reset
const SPI_RELOAD: u32 = CORTEX_HZ / SPI_HZ;

#[cortex_m_rt::entry]
fn main() -> ! {
    let board_p = bsp::Peripherals::take().unwrap();
    let cortex_p = cortex_m::Peripherals::take().unwrap();
    //let mut systick = Delay::with_source(cortext_m::Peripherals::take().unwrap().SYST, teensy4_bsp::EXT_SYSTICK_HZ, SystClkSource::External);
    let mut syst = cortex_p.SYST;
    syst.set_clock_source(SystClkSource::Core);

    syst.set_reload(SPI_RELOAD);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    // Reduce the number of pins to those specific
    // to the Teensy 4.1.
    let pins = bsp::pins::t41::from_pads(board_p.iomuxc);

    // See the `logging` module docs for more info.
    assert!(logging::init().is_ok());

    loop {
        
    }
}