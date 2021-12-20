use crate::packet_length::PacketLength;

#[derive(Debug)]
pub struct PacketHeader {
    pub version: u8,
    pub type_id: u8,
}

impl PacketHeader {
    pub fn from_bin_str(bin_str: &str) -> Self {
        let version = u8::from_str_radix(&bin_str[..3], 2).unwrap();
        let type_id = u8::from_str_radix(&bin_str[3..6], 2).unwrap();

        Self { version, type_id }
    }
}

impl PacketLength for PacketHeader {
    fn length(&self) -> usize {
        6
    }
}
