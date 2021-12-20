use crate::literal_packet::LiteralPacket;
use crate::operator_packet::OperatorPacket;
use crate::packet_header::PacketHeader;
use crate::packet_length::PacketLength;
use crate::packet_value::PacketValue;

#[derive(Debug)]
pub enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

impl Packet {
    pub fn from_bin_str(bin_str: &str) -> Self {
        let header = PacketHeader::from_bin_str(bin_str);

        match header.type_id {
            4 => Self::Literal(LiteralPacket::from_bin_str(header, &bin_str[6..])),
            _ => Self::Operator(OperatorPacket::from_bin_str(header, &bin_str[6..])),
        }
    }

    pub fn version_sum(&self) -> u32 {
        match self {
            Self::Literal(literal_packet) => literal_packet.header.version as u32,
            Self::Operator(operator_packet) => {
                operator_packet.header.version as u32
                    + operator_packet
                        .sub_packets
                        .iter()
                        .map(Packet::version_sum)
                        .sum::<u32>()
            }
        }
    }
}

impl PacketLength for Packet {
    fn length(&self) -> usize {
        match self {
            Self::Literal(literal_packet) => literal_packet.length(),
            Self::Operator(operator_packet) => operator_packet.length(),
        }
    }
}

impl PacketValue for Packet {
    fn value(&self) -> u64 {
        match self {
            Self::Literal(literal_packet) => literal_packet.value(),
            Self::Operator(operator_packet) => operator_packet.value(),
        }
    }
}

impl From<&str> for Packet {
    fn from(s: &str) -> Self {
        let mut binary = String::new();

        for hex in s.chars() {
            binary += match hex {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'A' => "1010",
                'B' => "1011",
                'C' => "1100",
                'D' => "1101",
                'E' => "1110",
                'F' => "1111",
                _ => panic!("Unknown hex: {}", hex),
            };
        }

        Self::from_bin_str(&binary)
    }
}
