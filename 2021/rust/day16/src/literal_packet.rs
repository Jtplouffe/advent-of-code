use crate::packet_header::PacketHeader;
use crate::packet_length::PacketLength;
use crate::packet_value::PacketValue;

#[derive(Debug)]
pub struct LiteralPacket {
    pub header: PacketHeader,
    body_length: usize,
    value: u64,
}

impl LiteralPacket {
    pub fn from_bin_str(header: PacketHeader, bin_str: &str) -> LiteralPacket {
        let mut binary_value = String::new();

        for i in (0..bin_str.len()).step_by(5) {
            binary_value += &bin_str[(i + 1)..=(i + 4)];
            if &bin_str[i..=i] == "0" {
                break;
            }
        }

        let body_length = (binary_value.len() as f32 * 1.25) as usize;

        Self {
            header,
            body_length,
            value: u64::from_str_radix(&binary_value, 2).unwrap(),
        }
    }
}

impl PacketLength for LiteralPacket {
    fn length(&self) -> usize {
        self.header.length() + self.body_length
    }
}

impl PacketValue for LiteralPacket {
    fn value(&self) -> u64 {
        self.value
    }
}
