pub fn hash(input: &str) -> u32 {
    input.chars().fold(0u32, |current_value, char| {
        ((current_value + char as u32) * 17) % 256
    })
}
