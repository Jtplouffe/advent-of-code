use crate::network::Network;

mod network;

const START: &str = "you";
const END: &str = "out";

const SERVER: &str = "svr";
const DAC: &str = "dac";
const FFT: &str = "fft";

fn main() {
    let input = include_str!("./input").trim();

    let network = Network::from(input);

    let part1 = network.different_path_count(START, END, None);
    println!("Part 1: {part1}");

    let part2 = network.different_path_count(SERVER, END, Some((DAC, FFT)));
    println!("Part 2: {part2}");
}
