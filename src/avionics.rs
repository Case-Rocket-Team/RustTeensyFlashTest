use imxrt_hal::{self, spi::SPI, iomuxc::gpio::Pin};
use teensy4_bsp as bsp;
use typenum::{UTerm, UInt, B1, B0};
use teensy4_bsp::t41::P1;

use crate::{flash::W25Q64, logging::logging};

pub struct AbstractAvionics<P: Pin> {
    pub flash: W25Q64<P>,
    pub spi: SPI<UInt<UInt<UInt<UTerm, B1>, B0>, B0>>,
    pub delayer: cortex_m::delay::Delay
}

pub type Avionics = AbstractAvionics<P1>;

pub trait HasAvionics {
    fn avionics(&self) -> &'static mut Avionics;
}

pub fn avionics() -> &'static mut Avionics {
    unsafe {
        &mut AVIONICS
    }
}

pub static mut AVIONICS: Avionics = {
    let mut board = imxrt_hal::Peripherals::take().unwrap();
    let cortex = cortex_m::Peripherals::take().unwrap();

    let pins = bsp::pins::t41::from_pads(board.iomuxc);

    let mut flash_cs_pin = bsp::hal::gpio::GPIO::new(pins.p1);//pins.p10);
    flash_cs_pin.set_fast(true);
    let flash_cs = flash_cs_pin.output();

    // See the `logging` module docs for more info.
    // (Provided by library)
    assert!(logging::init().is_ok());

    // Set the clock speed of the main core to
    // 600 MHz
    board.ccm.pll1.set_arm_clock(
        imxrt_hal::ccm::PLL1::ARM_HZ,
        &mut board.ccm.handle,
        &mut board.dcdc
    );

    // Set up the clock for SPI
    let (_, _, _, spi4_builder) = board.spi.clock(
        // Handle to CCM
        &mut board.ccm.handle,
        // See https://www.pjrc.com/teensy/IMXRT1060RM_rev2.pdf
        // Clock at 582 MHz
        imxrt_hal::ccm::spi::ClockSelect::Pll2,
        // Divide above clock speed by 8: 582/8 = 72 Mhz
        imxrt_hal::ccm::spi::PrescalarSelect::LPSPI_PODF_0,
    );
    
    let spi4 = spi4_builder.build(
        pins.p11, 
        pins.p12,
        pins.p13
    );
    
    AbstractAvionics {
        flash: W25Q64 {
            cs: flash_cs
        },
        spi: spi4,
        delayer: cortex_m::delay::Delay::with_source(
                cortex.SYST, 
                teensy4_bsp::EXT_SYSTICK_HZ,
                cortex_m::peripheral::syst::SystClkSource::External)
    }
};

impl<P: Pin> AbstractAvionics<P> {
    pub fn delay(&mut self, ms: u32) {
        self.delayer.delay_ms(ms)
    }

    pub fn log_bytes(bytes: &[u8]) {
        todo!()
    }

    pub fn log_byte(byte: u8) {
        todo!()
    }
}