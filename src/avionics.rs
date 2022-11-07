use core::ptr;

use imxrt_hal::{self, spi::SPI};
use teensy4_bsp as bsp;
use typenum::{UTerm, UInt, B1, B0};

use crate::{flash::W25Q64, logging};

pub struct Avionics {
    pub flash: W25Q64,
    pub spi: SPI<UInt<UInt<UInt<UTerm, B1>, B0>, B0>>,
    pub delayer: cortex_m::delay::Delay
}

pub trait HasAvionics {
    fn avionics(&self) -> &'static mut Avionics;
}

impl Avionics {
    pub fn take() -> Avionics {
        let mut board = imxrt_hal::Peripherals::take().unwrap();
        let cortex = cortex_m::Peripherals::take().unwrap();

        let pins = bsp::pins::t41::from_pads(board.iomuxc);

        let mut flash_cs_pin = bsp::hal::gpio::GPIO::new(pins.p10);
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

        let flash = W25Q64 {
            avionics: ptr::null_mut(),
            cs: flash_cs
        };

        let mut avionics = Avionics {
            flash,
            spi: spi4,
            delayer: cortex_m::delay::Delay::with_source(
                    cortex.SYST, 
                    teensy4_bsp::EXT_SYSTICK_HZ,
                    cortex_m::peripheral::syst::SystClkSource::External)
        };

        //flash.avionics = ptr::addr_of_mut!(avionics);
        avionics.flash.avionics = & mut avionics as * mut Avionics;

        avionics
    }

    pub fn delay(&mut self, ms: u32) {
        self.delayer.delay_ms(ms)
    }
}