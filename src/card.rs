use crate::color::Color;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Card {
    pub color: Color,
    pub value: CardValue,
}

#[derive(Debug)]
pub struct CardValue {
    pub gems: HashMap<Color, i32>,
}

impl CardValue {
    pub fn new() -> Self {
        CardValue {
            gems: HashMap::new(),
        }
    }

    /// Adds a gem to the card's value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cards_game_brex::card::CardValue;
    /// use cards_game_brex::color::Color;
    ///
    /// let mut card_value = CardValue::new();
    /// card_value.add_gem(Color::R, 5);
    /// assert_eq!(card_value.get_gem(&Color::R), 5);
    /// ```
    pub fn add_gem(&mut self, color: Color, amount: i32) {
        self.gems.insert(color, amount);
    }

    /// Gets the number of gems of a specific color.
    ///
    /// # Examples
    ///
    /// ```
    /// use cards_game_brex::card::CardValue;
    /// use cards_game_brex::color::Color;
    ///
    /// let mut card_value = CardValue::new();
    /// card_value.add_gem(Color::R, 5);
    /// assert_eq!(card_value.get_gem(&Color::R), 5);
    /// assert_eq!(card_value.get_gem(&Color::G), 0); // Color not added, should return 0
    /// ```
    pub fn get_gem(&self, color: &Color) -> i32 {
        *self.gems.get(color).unwrap_or(&0)
    }
}
