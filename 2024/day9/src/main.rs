use disk::Disk;

mod block;
mod disk;

fn main() {
    let input = include_str!("./input").trim_end();

    let mut disk1 = Disk::from_map(input);
    let mut disk2 = disk1.clone();

    disk1.compact_partial_files();
    println!("Part 1: {}", disk1.checksum());

    disk2.compact_whole_files();
    println!("Part 2: {}", disk2.checksum());
}
