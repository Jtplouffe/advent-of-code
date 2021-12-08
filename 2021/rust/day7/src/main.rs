use std::str::FromStr;

fn main() {
    let input = include_str!("input");

    let heights: Vec<_> = input.split(",").filter_map(|h| isize::from_str(h).ok()).collect();

    let min = heights.iter().min().unwrap();
    let max = heights.iter().max().unwrap();

    let mut fuels = Vec::<usize>::with_capacity((max - min) as usize);

    for i in *min..=*max {
        let fuel = heights.iter().map(|height| (*height - i).abs() as usize).sum();
        fuels.push(fuel);
    }
    println!("Part one: {}", fuels.iter().min().unwrap());

    fuels.clear();

    for i in *min..=*max {
        let fuel = heights.iter().map(|height| {
            let n = (*height - i).abs();
            (n * (n + 1) / 2) as usize
        }).sum();
        fuels.push(fuel);
    }
    println!("Part two: {}", fuels.iter().min().unwrap());
}
