use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin};
use crate::avionics::{Avionics, HasAvionics};

pub struct W25Q64 {
    pub avionics: * mut Avionics,
    pub cs: imxrt_hal::gpio::GPIO<teensy4_bsp::pins::t41::P1, imxrt_hal::gpio::Output>
}

// TODO: replace with macro
impl HasAvionics for W25Q64 {
    fn avionics(&self) -> &'static mut Avionics {
        unsafe {
            &mut *self.avionics
        }
    }
}

impl W25Q64 {
    #[inline(always)]
    pub fn select(&mut self) {
        self.cs.set_low().unwrap();
    }

    #[inline(always)]
    pub fn unselect(&mut self) {
        self.cs.set_high().unwrap();
    }
    
    pub fn send_instr<const TLENGTH: usize>(&mut self, mut bytes: [u8; TLENGTH]) -> [u8; TLENGTH] {
        self.select();
        self.avionics().spi.transfer(&mut bytes).unwrap();
        self.unselect();
        bytes
    }

    pub fn is_busy(&mut self) -> bool {
        (self.send_instr([0x05, 0x00])[1] & 0x01) == 1
    }

    pub fn block_until_ready(&mut self) {
        while self.is_busy() { }
    }

    pub fn read_manufacturer_and_device_id(&mut self) -> (u8, u8) {
        self.block_until_ready();
        let res = self.send_instr([0x90, 0x00, 0x00, 0x00, 0x00, 0x00]);
        (res[4], res[5])
    }

    pub fn write_enable(&mut self) {
        self.block_until_ready();
        self.send_instr([0x06]);
    }

    pub fn page_program<const TPROGRAMSIZE: usize>(&mut self, addr: u32, mut data: [u8; TPROGRAMSIZE]) {
        let mut part_1 =  [
            0x02_u8, 
            ((addr >> 16) & 0xff) as u8,
            ((addr >> 8) & 0xff) as u8,
            (addr & 0xff) as u8
        ];

        self.write_enable();
        self.block_until_ready();
        self.select();
        self.avionics().spi.transfer(&mut part_1).ok();
        self.avionics().spi.transfer(&mut data).ok();
        self.unselect();
    }

    pub fn read_data<const TDATALENGTH: usize>(&mut self, addr: u32) -> [u8; TDATALENGTH] {
        let mut part_1 =  [
            0x03_u8, 
            ((addr >> 16) & 0xff) as u8,
            ((addr >> 8) & 0xff) as u8,
            (addr & 0xff) as u8
        ];

        let mut received = [0u8; TDATALENGTH];

        self.block_until_ready();
        self.select();
        self.avionics().spi.transfer(&mut part_1).unwrap();
        self.avionics().spi.transfer(&mut received).unwrap();
        self.unselect();

        received
    }

    pub fn erase_sector(&mut self, addr: u32) {
        let instr =  [
            0x20_u8, 
            ((addr >> 16) & 0xff) as u8,
            ((addr >> 8) & 0xff) as u8,
            (addr & 0xff) as u8
        ];

        self.write_enable();
        self.block_until_ready();
        self.send_instr(instr);
    }
}
