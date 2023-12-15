#[derive(Clone)]
pub struct Lens<'a> {
    pub label: &'a str,
    pub focal_length: u8,
}
