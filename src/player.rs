use crate::card::Card;
use crate::color::Color;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Player {
    pub wallet: HashMap<Color, i32>,
    pub deck: Vec<Card>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            wallet: HashMap::new(),
            deck: vec![],
        }
    }

    pub fn can_purchase(&self, card: &Card) -> bool {
        for (color, amount) in &card.value.gems {
            let available = self.wallet.get(color).unwrap_or(&0);
            if available < amount {
                return false;
            }
        }
        true
    }

    pub fn purchase(&mut self, card: Card) -> bool {
        if !self.can_purchase(&card) {
            return false;
        }
        for (color, amount) in &card.value.gems {
            let available = self.wallet.entry(*color).or_insert(0);
            *available -= amount;
        }
        self.deck.push(card);
        true
    }
}
