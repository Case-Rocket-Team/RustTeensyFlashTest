#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

mod logging;

#[cortex_m_rt::entry]
fn main() -> ! {
    log::info!("Starting Teensy");
    let board_p = bsp::Peripherals::take().unwrap();
    let cortex_p = cortex_m::Peripherals::take().unwrap();

    // Reduce the number of pins to those specific
    // to the Teensy 4.1.
    let pins = bsp::pins::t41::from_pads(board_p.iomuxc);

    // See the `logging` module docs for more info.
    assert!(logging::init().is_ok());

    // Addresses 0x70000000 and above go to the flash chip.
    const ADDR: u32 = 0x70000000;
    const TEST_INT: u32 = 123;
    let test_int_ptr: * mut u32 = ADDR as *mut u32;

    let mut success = false;
    
    unsafe {
        *test_int_ptr = TEST_INT;
        *test_int_ptr += 1;
        log::info!("Value on chip is: {}", *test_int_ptr);
        success = *test_int_ptr == TEST_INT;
    }

    loop {
        if success {
            log::info!("Successful R/W!");
        } else {
            log::info!("Failure :(")
        }
    }
}