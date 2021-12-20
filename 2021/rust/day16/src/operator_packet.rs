use crate::operation::Operation;
use crate::packet::Packet;
use crate::packet_header::PacketHeader;
use crate::packet_length::PacketLength;
use crate::packet_value::PacketValue;

#[derive(Debug)]
pub struct OperatorPacket {
    pub header: PacketHeader,
    length_type_id: u8,
    operation: Operation,
    pub sub_packets: Vec<Packet>,
}

impl OperatorPacket {
    pub fn from_bin_str(header: PacketHeader, bin_str: &str) -> Self {
        let length_type_id = u8::from_str_radix(&bin_str[..1], 2).unwrap();
        let operation = Operation::new(header.type_id);
        let body = &bin_str[1..];

        let sub_packets = match length_type_id {
            0 => {
                let body_length = u32::from_str_radix(&body[..15], 2).unwrap() as usize;
                let body = &body[15..(15 + body_length)];
                let mut pos = 0;

                let mut sub_packets = Vec::new();
                loop {
                    let sub_packet = Packet::from_bin_str(&body[pos..]);
                    pos += sub_packet.length();
                    sub_packets.push(sub_packet);

                    if pos >= body_length {
                        break;
                    }
                }

                sub_packets
            }
            1 => {
                let sub_packet_count = usize::from_str_radix(&body[..11], 2).unwrap();
                let body = &body[11..];

                let mut sub_packets = Vec::new();

                let mut pos = 0;
                for _ in 0..sub_packet_count {
                    let sub_packet = Packet::from_bin_str(&body[pos..]);
                    pos += sub_packet.length();
                    sub_packets.push(sub_packet);
                }

                sub_packets
            }
            _ => panic!("Unknown length type id: {}", length_type_id),
        };

        Self {
            header,
            length_type_id,
            operation,
            sub_packets,
        }
    }
}

impl PacketLength for OperatorPacket {
    fn length(&self) -> usize {
        self.header.length()
            + 1
            + self.sub_packets.iter().map(Packet::length).sum::<usize>()
            + match self.length_type_id {
                0 => 15,
                1 => 11,
                _ => panic!("Unknown length type: {}", self.length_type_id),
            }
    }
}

impl PacketValue for OperatorPacket {
    fn value(&self) -> u64 {
        match self.operation {
            Operation::Sum => self.sub_packets.iter().map(Packet::value).sum(),
            Operation::Product => self.sub_packets.iter().map(Packet::value).product(),
            Operation::Minimum => self.sub_packets.iter().map(Packet::value).min().unwrap(),
            Operation::Maximum => self.sub_packets.iter().map(Packet::value).max().unwrap(),
            Operation::GreaterThan => {
                if self.sub_packets[0].value() > self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            Operation::LessThan => {
                if self.sub_packets[0].value() < self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            Operation::EqualTo => {
                if self.sub_packets[0].value() == self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}
