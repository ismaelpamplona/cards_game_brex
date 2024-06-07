#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub enum Color {
    G,
    R,
    B,
    Y,
    P,
}

impl Color {
    /// Converts the color to a string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use cards_game_brex::color::Color;
    ///
    /// let color = Color::G;
    /// assert_eq!(color.as_str(), "Green");
    /// ```
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
