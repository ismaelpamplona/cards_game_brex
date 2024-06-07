# Card Game Challenge - Brex

## Context

You have a card game where each card has a color and an acquisition cost in gems. The card colors can be: G (Green), B (Blue), R (Red), Y (Yellow), P (Purple). The acquisition cost is represented by pairs of numbers and letters indicating the required quantity of gems of each color.

## Example of a Card

- Top right corner: G
- Bottom left corner: 1R2G3B (1 red gem, 2 green gems, 3 blue gems)

  ```
  +-------------+
  |           G |
  |             |
  |             |
  |             |
  |             |
  |             |
  | 1R2G3B      |
  +-------------+
  ```

## Requirements

- The player has a balance of gems and a deck of cards.
- The gem balance and the deck of cards are independent.

## Challenges

1. **Function `can_purchase`**:
   Develop a function/method `can_purchase` that checks if a player can purchase a card based on their available gem balance.

2. **Function `purchase`**:
   Develop a function/method `purchase` that deducts the gem balance of the player when purchasing a card and adds the card to the player's deck, provided they have sufficient balance.
