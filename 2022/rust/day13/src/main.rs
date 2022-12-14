use std::time::Instant;

use crate::packet::Packet;

mod packet;

fn main() {
    let i = Instant::now();
    let input = include_str!("./input");

    let mut packets = input
        .replace("\n\n", "\n")
        .lines()
        .map(Packet::from)
        .collect::<Vec<_>>();

    let valid_order_packet_indices_sum = packets
        .chunks(2)
        .enumerate()
        .filter_map(|(index, packets)| {
            if packets[0].is_before(&packets[1]) {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("Part 1: {valid_order_packet_indices_sum}");

    let divider_packets = vec![
        Packet::List(vec![Packet::List(vec![Packet::Value(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Value(6)])]),
    ];
    packets.extend_from_slice(&divider_packets);
    packets.sort_unstable();

    let decoder_key = packets
        .iter()
        .enumerate()
        .filter_map(|(index, packet)| {
            if divider_packets.contains(packet) {
                Some(index + 1)
            } else {
                None
            }
        })
        .product::<usize>();
    println!("Part 2: {decoder_key}");

    println!("{:?}", i.elapsed());
}
