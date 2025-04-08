use crate::card::Card;
use crate::evaluator7::evaluate_7cards;
use crate::rank::{describe_rank_category, get_rank_category, RankCategory};
use rand::seq::SliceRandom;
use std::collections::HashMap;

// Number of simulations to run for Monte Carlo calculations
const NUM_SIMULATIONS: usize = 10000;

// Calculate probabilities of getting each hand category with the current cards
pub fn calculate_hand_probabilities(
    hole_cards: &[Card],
    community_cards: &[Card],
    num_players: usize,
    pack_count: usize,
) -> HashMap<RankCategory, f64> {
    let mut rng = rand::thread_rng();
    let mut results = HashMap::new();

    // Create a deck with the specified number of packs
    let mut deck = Vec::new();
    for _ in 0..pack_count {
        for rank in 0..13 {
            for suit in 0..4 {
                deck.push(Card::from_id(rank * 4 + suit));
            }
        }
    }

    // Remove known cards from the deck
    let known_cards: Vec<Card> = hole_cards
        .iter()
        .chain(community_cards.iter())
        .cloned()
        .collect();
    deck.retain(|card| {
        !known_cards
            .iter()
            .any(|known_card| known_card.id_ == card.id_)
    });

    // Run simulations
    for _ in 0..NUM_SIMULATIONS {
        // Shuffle the deck
        let mut shuffled_deck = deck.clone();
        shuffled_deck.shuffle(&mut rng);

        // Complete the community cards if needed
        let mut sim_community_cards = community_cards.to_vec();
        while sim_community_cards.len() < 5 {
            sim_community_cards.push(shuffled_deck.pop().unwrap());
        }

        // Create a 7-card hand
        let mut hand = hole_cards.to_vec();
        hand.extend(sim_community_cards.iter().cloned());

        // Extract card IDs for evaluation
        let card_ids: Vec<i32> = hand.iter().map(|card| card.id_).collect();

        // Evaluate the hand
        let rank = evaluate_7cards(
            card_ids[0],
            card_ids[1],
            card_ids[2],
            card_ids[3],
            card_ids[4],
            card_ids[5],
            card_ids[6],
        );
        let category = get_rank_category(rank);

        // Increment the counter for this category
        *results.entry(category).or_insert(0.0) += 1.0;
    }

    // Convert counts to probabilities
    for (_, count) in results.iter_mut() {
        *count = *count / NUM_SIMULATIONS as f64 * 100.0;
    }

    results
}

// Helper function to get ordered probabilities
pub fn get_ordered_probabilities(probs: &HashMap<RankCategory, f64>) -> Vec<(RankCategory, f64)> {
    let mut ordered = vec![
        (
            RankCategory::StraightFlush,
            *probs.get(&RankCategory::StraightFlush).unwrap_or(&0.0),
        ),
        (
            RankCategory::FourOfAKind,
            *probs.get(&RankCategory::FourOfAKind).unwrap_or(&0.0),
        ),
        (
            RankCategory::FullHouse,
            *probs.get(&RankCategory::FullHouse).unwrap_or(&0.0),
        ),
        (
            RankCategory::Flush,
            *probs.get(&RankCategory::Flush).unwrap_or(&0.0),
        ),
        (
            RankCategory::Straight,
            *probs.get(&RankCategory::Straight).unwrap_or(&0.0),
        ),
        (
            RankCategory::ThreeOfAKind,
            *probs.get(&RankCategory::ThreeOfAKind).unwrap_or(&0.0),
        ),
        (
            RankCategory::TwoPair,
            *probs.get(&RankCategory::TwoPair).unwrap_or(&0.0),
        ),
        (
            RankCategory::OnePair,
            *probs.get(&RankCategory::OnePair).unwrap_or(&0.0),
        ),
        (
            RankCategory::HighCard,
            *probs.get(&RankCategory::HighCard).unwrap_or(&0.0),
        ),
    ];
    ordered.retain(|&(_, prob)| prob > 0.0);
    ordered
}

// Calculate the probability of winning with the current cards
pub fn calculate_win_probability(
    hole_cards: &[Card],
    community_cards: &[Card],
    num_players: usize,
    pack_count: usize,
) -> f64 {
    let mut rng = rand::thread_rng();
    let mut wins = 0.0;

    // Create a deck with the specified number of packs
    let mut deck = Vec::new();
    for _ in 0..pack_count {
        for rank in 0..13 {
            for suit in 0..4 {
                deck.push(Card::from_id(rank * 4 + suit));
            }
        }
    }

    // Remove known cards from the deck
    let known_cards: Vec<Card> = hole_cards
        .iter()
        .chain(community_cards.iter())
        .cloned()
        .collect();
    deck.retain(|card| {
        !known_cards
            .iter()
            .any(|known_card| known_card.id_ == card.id_)
    });

    // Run simulations
    for _ in 0..NUM_SIMULATIONS {
        // Shuffle the deck
        let mut shuffled_deck = deck.clone();
        shuffled_deck.shuffle(&mut rng);

        // Complete the community cards if needed
        let mut sim_community_cards = community_cards.to_vec();
        while sim_community_cards.len() < 5 {
            sim_community_cards.push(shuffled_deck.pop().unwrap());
        }

        // Create opponent hands
        let mut opponent_hands = Vec::new();
        for _ in 0..num_players - 1 {
            let mut opponent_hand = Vec::new();
            opponent_hand.push(shuffled_deck.pop().unwrap());
            opponent_hand.push(shuffled_deck.pop().unwrap());
            opponent_hands.push(opponent_hand);
        }

        // Evaluate our hand
        let mut our_hand = hole_cards.to_vec();
        our_hand.extend(sim_community_cards.iter().cloned());

        // Extract card IDs for our hand
        let our_card_ids: Vec<i32> = our_hand.iter().map(|card| card.id_).collect();
        let our_rank = evaluate_7cards(
            our_card_ids[0],
            our_card_ids[1],
            our_card_ids[2],
            our_card_ids[3],
            our_card_ids[4],
            our_card_ids[5],
            our_card_ids[6],
        );

        // Check if we win against all opponents
        let mut won = true;
        for opponent_hand in opponent_hands {
            let mut full_opponent_hand = opponent_hand;
            full_opponent_hand.extend(sim_community_cards.iter().cloned());

            // Extract card IDs for opponent hand
            let opponent_card_ids: Vec<i32> =
                full_opponent_hand.iter().map(|card| card.id_).collect();
            let opponent_rank = evaluate_7cards(
                opponent_card_ids[0],
                opponent_card_ids[1],
                opponent_card_ids[2],
                opponent_card_ids[3],
                opponent_card_ids[4],
                opponent_card_ids[5],
                opponent_card_ids[6],
            );

            if opponent_rank < our_rank {
                won = false;
                break;
            }
        }

        if won {
            wins += 1.0;
        }
    }

    // Return win probability as a percentage
    wins / NUM_SIMULATIONS as f64 * 100.0
}

// Implement necessary traits for RankCategory
impl std::hash::Hash for RankCategory {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (*self as i32).hash(state);
    }
}

impl PartialEq for RankCategory {
    fn eq(&self, other: &Self) -> bool {
        *self as i32 == *other as i32
    }
}

impl Eq for RankCategory {}

// Implement Copy and Clone for RankCategory
impl Copy for RankCategory {}
impl Clone for RankCategory {
    fn clone(&self) -> Self {
        *self
    }
}

// Implement Display for RankCategory
impl std::fmt::Display for RankCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = describe_rank_category(*self);
        if description.is_empty() {
            write!(f, "Unknown")
        } else {
            write!(f, "{}", description)
        }
    }
}
