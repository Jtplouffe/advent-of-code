pub fn gcd(left: usize, right: usize) -> usize {
    let mut max = left;
    let mut min = right;
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

pub fn lcm(left: usize, right: usize) -> usize {
    left * right / gcd(left, right)
}
