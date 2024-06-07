#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum Color {
    G,
    R,
    B,
    Y,
    P,
}

impl Color {
    pub fn as_str(&self) -> &str {
        match self {
            Color::G => "Green",
            Color::R => "Red",
            Color::B => "Blue",
            Color::Y => "Yellow",
            Color::P => "Purple",
        }
    }
}
