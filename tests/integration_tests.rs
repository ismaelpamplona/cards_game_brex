use cards_game_brex::card::{Card, CardValue};
use cards_game_brex::color::Color;
use cards_game_brex::player::Player;
use std::collections::HashMap;

#[test]
fn test_player_can_purchase_card() {
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
fn test_player_purchase_card_success() {
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
fn test_player_purchase_card_failure() {
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
    assert_eq!(player.wallet.get(&Color::R), Some(&0));
    assert_eq!(player.wallet.get(&Color::G), Some(&1));
    assert_eq!(player.wallet.get(&Color::B), Some(&1));
    assert_eq!(player.deck.len(), 0);
}
