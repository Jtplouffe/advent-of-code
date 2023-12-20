use network::Network;

mod modules;
mod network;
mod pulse;

fn main() {
    let input = include_str!("./input");

    let mut network = Network::from(input);

    let signal_product = network.button_push_signal_count_product(1000);
    println!("Part 1: {signal_product}");
}
