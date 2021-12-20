use std::time::Instant;
use crate::packet::Packet;
use crate::packet_value::PacketValue;

mod literal_packet;
mod operation;
mod operator_packet;
mod packet;
mod packet_header;
mod packet_length;
mod packet_value;

fn main() {
    let i = Instant::now();
    let input = include_str!("input");
    let root_packet = Packet::from(input);
    println!("Part one: {}", root_packet.version_sum());
    println!("Part two: {}", root_packet.value());
    println!("{:?}", i.elapsed());
}
