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

## Instructions to Run the Project

### Prerequisites

- Ensure you have [Docker](https://www.docker.com/) installed on your system.
- Ensure you have [Docker Compose](https://docs.docker.com/compose/) installed on your system.

### Running the tests with docker

1. **Clone the Repository**

2. **Build the Docker Image**:
   Build the Docker image using Docker Compose:

   ```sh
   docker compose build
   ```

3. **Run the the tests in Docker Container**:
   Run the Docker container using Docker Compose:
   ```sh
   docker compose up
   ```

### Running the tests with cargo

All tests:

```sh
cargo test
```

Specific tests:

```sh
cargo test --test <test_name>
```

### Generating docs

```sh
cargo doc --open
```
