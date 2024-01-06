use crate::{
    card::{Card, CardType, CardValue},
    deck::Deck,
    player::Player,
};

#[derive(Debug)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}
fn find_pairs(cards: &Vec<Card>) -> Vec<CardValue> {
    println!("{:?}", &cards);
    let mut seen_values = Vec::new();
    let mut pairs = Vec::new();

    for card in cards {
        if seen_values.contains(&card.card_value) {
            // This card value has already been seen, it's a pair
            pairs.push(card.card_value);
        } else {
            // Add the card value to the list of seen values
            seen_values.push(card.card_value);
        }
    }

    pairs
}
// Function to convert CardValue to a comparable type (u8)
fn card_value_to_u8(card_value: &CardValue) -> u8 {
    match *card_value {
        CardValue::Ace => 1,
        CardValue::Number(n) => n,
        CardValue::Jack => 11,
        CardValue::Queen => 12,
        CardValue::King => 13,
    }
}
fn decrement_card_value(value: CardValue) -> CardValue {
    match value {
        CardValue::Number(n) if n > 2 => CardValue::Number(n - 1),
        CardValue::Jack => CardValue::Number(10),
        CardValue::Queen => CardValue::Jack,
        CardValue::King => CardValue::Queen,
        CardValue::Ace => CardValue::King,
        _ => value,
    }
}
fn find_royal_flush(cards: &Vec<Card>) -> bool {
    if !(cards.len() < 5) {
        // Clone and sort the cards by rank in descending order
        let mut sorted_cards = cards.clone();
        sorted_cards.sort_by_key(|card| card_value_to_u8(&card.card_value));

        // Check if there is a sequence of five cards with the highest values
        let mut consecutive_count = 0;
        let mut current_value = CardValue::Ace; // Start with the highest value

        for card in sorted_cards.iter() {
            if card.card_value == current_value && consecutive_count < 5 {
                consecutive_count += 1;
                current_value = decrement_card_value(current_value);
            } else if card.card_value != current_value {
                // Reset consecutive count if the current card doesn't match the expected value
                consecutive_count = 1;
                current_value = decrement_card_value(CardValue::Ace); // Reset to Ace
            }
        }

        consecutive_count == 5 // Check if a sequence of five cards was found
    } else {
        false
    }
}

fn find_straight_flush(cards: &Vec<Card>) -> bool {
    if !(cards.len() < 5) {
        // Sort the cards by rank in descending order
        let mut sorted_cards = cards.clone();
        sorted_cards.sort_by_key(|card| card_value_to_u8(&card.card_value));

        // Check if there is a sequence of five consecutive cards with the same suit
        let mut consecutive_count = 0;
        let mut current_value = sorted_cards[0].card_value; // Start with the highest value
        let mut current_suit = sorted_cards[0].card_type;

        for card in sorted_cards {
            if card.card_value == current_value
                && card.card_type == current_suit
                && consecutive_count < 5
            {
                consecutive_count += 1;
                current_value = decrement_card_value(current_value);
            } else if card.card_value != current_value || card.card_type != current_suit {
                // Reset consecutive count if the current card doesn't match the expected value or suit
                consecutive_count = 1;
                current_value = decrement_card_value(card.card_value); // Reset to the current card value
                current_suit = card.card_type;
            }
        }

        consecutive_count == 5 // Check if a sequence of five consecutive cards with the same suit was found
    } else {
        false
    }
}
fn find_four_of_a_kind(cards: &Vec<Card>) -> bool {
    if !(cards.len() < 4) {
        // Count occurrences of each card value
        let mut value_counts = std::collections::HashMap::new();

        for card in cards {
            *value_counts.entry(card.card_value).or_insert(0) += 1;
        }

        // Check if there is a card value with count 4
        value_counts.values().any(|&count| count == 4)
    } else {
        false
    }
}
fn find_full_house(cards: &Vec<Card>) -> bool {
    if !(cards.len() < 5) {
        // Count occurrences of each card value
        let mut value_counts = std::collections::HashMap::new();

        for card in cards {
            *value_counts.entry(card.card_value).or_insert(0) += 1;
        }

        // Check if there is a three-of-a-kind and a pair
        let has_three_of_a_kind = value_counts.values().any(|&count| count == 3);
        let has_pair = value_counts.values().any(|&count| count == 2);

        has_three_of_a_kind && has_pair
    } else {
        false
    }
}
fn find_flush(cards: &Vec<Card>) -> bool {
    if !(cards.len() < 5) {
        // Check if all cards have the same suit
        let first_suit = cards
            .first()
            .map_or(CardType::Clover, |card| card.card_type);
        cards.iter().all(|card| card.card_type == first_suit)
    } else {
        false
    }
}
fn find_straight(cards: &Vec<Card>) -> bool {
    if !(cards.len() < 5) {
        // Sort the cards by rank in descending order
        let mut sorted_cards = cards.clone();
        sorted_cards.sort_by_key(|card| card_value_to_u8(&card.card_value));

        // Check if there is a sequence of five consecutive card values
        let mut consecutive_count = 0;
        let mut current_value = sorted_cards[0].card_value;

        for card in sorted_cards {
            if card.card_value == current_value && consecutive_count < 5 {
                consecutive_count += 1;
                current_value = decrement_card_value(current_value);
            } else if card.card_value != current_value {
                // Reset consecutive count if the current card doesn't match the expected value
                consecutive_count = 1;
                current_value = decrement_card_value(card.card_value); // Reset to the current card value
            }
        }

        consecutive_count == 5 // Check if a sequence of five consecutive card values was found
    } else {
        false
    }
}
fn find_three_of_a_kind(cards: &Vec<Card>) -> bool {
    // Count occurrences of each card value
    let mut value_counts = std::collections::HashMap::new();

    for card in cards {
        *value_counts.entry(card.card_value).or_insert(0) += 1;
    }

    // Check if there is a card value with count 3
    value_counts.values().any(|&count| count == 3)
}
fn find_high_card(cards: &Vec<Card>) -> Option<CardValue> {
    // Find the card with the highest value
    cards.iter().map(|card| card.card_value).max()
}
pub struct Game {
    pub player_count: u8,
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
    pub community_cards: Vec<Card>,
    pub turn_count: u8,
}
impl Game {
    pub fn new(player_count: u8) -> Game {
        let mut new_deck = Deck::new(4);
        Deck::shuffle_deck(&mut new_deck);
        let mut game = Game {
            player_count: player_count,
            players: (0..player_count).map(|_| Player::new()).collect(),
            deck: new_deck,
            community_cards: Vec::new(),
            turn_count: 0,
        };
        game.distribute_cards_to_players();
        game
    }
    pub fn distribute_cards_to_players(&mut self) {
        // Ensure there are enough cards in the deck for all players
        if self.deck.len() < self.player_count as usize * 2 {
            panic!("Not enough cards in the deck to distribute to players.");
        }

        // Distribute cards to each player
        for player in &mut self.players {
            for _ in 0..2 {
                if let Some(card) = self.deck.pop() {
                    player.add_card(card);
                }
            }
        }
    }

    pub fn check_hand_rank(&self, cards: &Vec<Card>) -> Vec<HandRank> {
        let mut handranks: Vec<HandRank> = Vec::new(); // Initialize the vector
        let pair_count = find_pairs(&cards).len();

        let mut concat_cards = cards.clone();
        concat_cards.extend(self.community_cards.clone());
        if find_royal_flush(cards) {
            handranks.push(HandRank::RoyalFlush);
        } else if find_straight_flush(cards) {
            handranks.push(HandRank::StraightFlush);
        } else if find_four_of_a_kind(cards) {
            handranks.push(HandRank::FourOfAKind);
        } else if find_full_house(cards) {
            handranks.push(HandRank::FullHouse);
        } else if find_flush(cards) {
            handranks.push(HandRank::Flush);
        } else if find_straight(cards) {
            handranks.push(HandRank::Straight);
        } else if find_three_of_a_kind(cards) {
            handranks.push(HandRank::ThreeOfAKind);
        } else if pair_count == 2 {
            handranks.push(HandRank::TwoPair);
        } else if pair_count == 1 {
            handranks.push(HandRank::OnePair);
        } else {
            handranks.push(HandRank::HighCard);
        }

        handranks // Return the vector
    }
    fn release_card(&mut self) {
        // Ensure there are enough cards in the deck for all players
        if self.deck.len() < self.player_count as usize * 2 {
            panic!("Not enough cards in the deck to distribute to players.");
        }

        if let Some(card) = self.deck.pop() {
            self.community_cards.push(card);
        }
    }
    pub fn play_turn(&mut self) {
        if self.turn_count == 0 {
            //Release 3 cards
            self.release_card();
            self.release_card();
            self.release_card();
        }
        if self.turn_count == 1 {
            self.release_card();
        }
        if self.turn_count == 2 {
            self.release_card();
        }

        self.release_card();
        println!("{:?}", self.community_cards.last());
        self.turn_count += 1;
    }
}
