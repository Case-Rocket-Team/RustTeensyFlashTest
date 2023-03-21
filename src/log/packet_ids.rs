pub struct PacketIDs;

macro_rules! packet_ids {
    ($($val:tt $ident:ident)*) => {
        impl PacketIDs {
            $(pub const $ident: u8 = $val;)*
        }        
    }
}

packet_ids! {
    0x00 NULL
    0x01 ANNOTATION
    0x02 I32
    0x03 F64
}
