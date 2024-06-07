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

    pub fn add_gem(&mut self, color: Color, amount: i32) {
        self.gems.insert(color, amount);
    }

    pub fn get_gem(&self, color: &Color) -> i32 {
        *self.gems.get(color).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_card_value_new() {
        let card_value = CardValue::new();
        assert!(card_value.gems.is_empty());
    }

    #[test]
    fn test_card_value_add_gem() {
        let mut card_value = CardValue::new();
        card_value.add_gem(Color::R, 5);
        assert_eq!(card_value.get_gem(&Color::R), 5);
    }

    #[test]
    fn test_card_value_get_gem_default() {
        let card_value = CardValue::new();
        assert_eq!(card_value.get_gem(&Color::R), 0);
    }

    #[test]
    fn test_card_value_get_gem_after_add() {
        let mut card_value = CardValue::new();
        card_value.add_gem(Color::R, 5);
        card_value.add_gem(Color::G, 3);
        assert_eq!(card_value.get_gem(&Color::R), 5);
        assert_eq!(card_value.get_gem(&Color::G), 3);
        assert_eq!(card_value.get_gem(&Color::B), 0);
    }

    #[test]
    fn test_card_creation() {
        let mut card_value = CardValue::new();
        card_value.add_gem(Color::R, 1);
        card_value.add_gem(Color::G, 2);
        card_value.add_gem(Color::B, 3);

        let card = Card {
            color: Color::G,
            value: card_value,
        };

        assert_eq!(card.color, Color::G);
        assert_eq!(card.value.get_gem(&Color::R), 1);
        assert_eq!(card.value.get_gem(&Color::G), 2);
        assert_eq!(card.value.get_gem(&Color::B), 3);
    }
}
