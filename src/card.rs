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
