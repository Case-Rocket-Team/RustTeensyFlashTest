use core::{mem::size_of, slice, iter};

use crate::avionics::{avionics, Avionics};

use super::packet_ids::PacketIDs;

pub trait Loggable {
    const PACKET_ID: u8;
    type Iter: Iterator<Item = u8>;
    
    /// Convert to bytes, including the packet ID,
    /// in the format for logging
    fn log_byte_iter(&self) -> Self::Iter;

    fn log(&self) {
        for byte in self.log_byte_iter() {
            Avionics::log_byte(byte)
        }
    }

    fn log_with_msg(&self, annotation: &str) {
        self.log();
        annotation.log();
    }
}

pub trait SizedLoggable: Loggable where Self: Sized {
    /// Size of the item, in bytes
    /// Including the packet ID
    const SIZE: usize;
}

enum LogStrIteratorStage {
    FirstByte,
    AnnotationByte,
    LengthByte,
    DataByte
}

struct LogStrIterator<'a> {
    str_iter: slice::Iter<'a, u8>,
    count: usize,
    length_remaining: usize,
    stage: LogStrIteratorStage
}

impl LogStrIterator<'_> {
    fn new<'a>(str: &'a str) -> LogStrIterator<'a> {
        LogStrIterator { 
            str_iter: str.as_bytes().iter(), 
            count: 0, 
            length_remaining: str.as_bytes().len(), 
            stage: LogStrIteratorStage::FirstByte
        }
    }
}

impl<'a> Iterator for LogStrIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stage {
            LogStrIteratorStage::FirstByte => {
                if self.length_remaining == 0 {
                    None
                } else {
                    self.stage = LogStrIteratorStage::AnnotationByte;
                    Some(PacketIDs::NULL)
                }
                
            },
            LogStrIteratorStage::AnnotationByte => {
                if self.length_remaining == 0 {
                    None
                } else {
                    self.stage = LogStrIteratorStage::LengthByte;
                    Some(PacketIDs::ANNOTATION)
                }
            },
            LogStrIteratorStage::LengthByte => {
                self.stage = LogStrIteratorStage::DataByte;
                Some(self.length_remaining.min(255) as u8)
            },
            LogStrIteratorStage::DataByte => {
                self.count += 1;
                self.length_remaining -= 1;

                if self.count >= 255 {
                    self.stage = LogStrIteratorStage::AnnotationByte;
                    self.count = 0;
                }

                self.str_iter.next().copied()
            }
        }
    }
}

impl<'a> Loggable for &'a str {
    const PACKET_ID: u8 = PacketIDs::NULL;
    type Iter = LogStrIterator<'a>;

    fn log_byte_iter(&self) -> Self::Iter {
        LogStrIterator::new(&self)
    }
}

impl Loggable for i32 {
    const PACKET_ID: u8 = PacketIDs::I32;
    type Iter = iter::Chain<iter::Once<u8>, core::array::IntoIter<u8, 4>>;

    fn log_byte_iter(&self) -> Self::Iter {
        iter::once(Self::PACKET_ID).chain(self.to_be_bytes().into_iter())
    }
}

impl SizedLoggable for i32 {
    const SIZE: usize = size_of::<i32>() + 1;
}