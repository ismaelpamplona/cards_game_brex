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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardValue;

    #[test]
    fn test_can_purchase_success() {
        let mut player = Player::new();
        player.wallet.insert(Color::R, 5);
        player.wallet.insert(Color::G, 3);
        player.wallet.insert(Color::B, 4);

        let mut card_value = CardValue::new();
        card_value.add_gem(Color::R, 1);
        card_value.add_gem(Color::G, 2);
        card_value.add_gem(Color::B, 3);

        let card = Card {
            color: Color::G,
            value: card_value,
        };

        assert!(player.can_purchase(&card));
    }

    #[test]
    fn test_can_purchase_failure() {
        let mut player = Player::new();
        player.wallet.insert(Color::R, 0);
        player.wallet.insert(Color::G, 1);
        player.wallet.insert(Color::B, 1);

        let mut card_value = CardValue::new();
        card_value.add_gem(Color::R, 1);
        card_value.add_gem(Color::G, 2);
        card_value.add_gem(Color::B, 3);

        let card = Card {
            color: Color::G,
            value: card_value,
        };

        assert!(!player.can_purchase(&card));
    }

    #[test]
    fn test_purchase_success() {
        let mut player = Player::new();
        player.wallet.insert(Color::R, 5);
        player.wallet.insert(Color::G, 3);
        player.wallet.insert(Color::B, 4);

        let mut card_value = CardValue::new();
        card_value.add_gem(Color::R, 1);
        card_value.add_gem(Color::G, 2);
        card_value.add_gem(Color::B, 3);

        let card = Card {
            color: Color::G,
            value: card_value,
        };

        assert!(player.purchase(card));
        assert_eq!(player.wallet.get(&Color::R), Some(&4));
        assert_eq!(player.wallet.get(&Color::G), Some(&1));
        assert_eq!(player.wallet.get(&Color::B), Some(&1));
        assert_eq!(player.deck.len(), 1);
    }

    #[test]
    fn test_purchase_failure() {
        let mut player = Player::new();
        player.wallet.insert(Color::R, 0);
        player.wallet.insert(Color::G, 1);
        player.wallet.insert(Color::B, 1);

        let mut card_value = CardValue::new();
        card_value.add_gem(Color::R, 1);
        card_value.add_gem(Color::G, 2);
        card_value.add_gem(Color::B, 3);

        let card = Card {
            color: Color::G,
            value: card_value,
        };

        assert!(!player.purchase(card));
        assert_eq!(player.deck.len(), 0);
    }
}
