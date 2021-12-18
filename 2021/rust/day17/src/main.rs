use crate::area::Area;
use crate::probe_launcher::ProbeLauncher;

mod area;
mod probe;
mod probe_launcher;

fn main() {
    let probe_launcher = ProbeLauncher::new(Area::new((277, -92), (318, -53)));

    println!("Part one: {}", probe_launcher.highest_possible_height());
    println!("Part two: {}", probe_launcher.target_hit_velocity_count());
}
