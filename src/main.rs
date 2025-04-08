// lib.rs
mod card;
mod deck;
mod dptables;
mod evaluator;
mod evaluator7;
mod game;
mod hand;
mod hash;
mod hash_table7;
mod hashtable;
mod player;
mod probability;
mod rank;
mod seven_four_six_two;

use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::io::{self, Read};

// Export relevant types for external use
pub use card::Card;
pub use deck::Deck;
use evaluator7::evaluate_7cards;
pub use game::Game;
pub use player::Player;
use probability::{calculate_hand_probabilities, calculate_win_probability};
use rank::{describe_rank_category, get_rank_category, RankCategory};

fn get_rank_of_7_perfect(cards: Vec<&str>) -> i32 {
    // &Card::from_name("Tc".to_string()),
    // &Card::from_name("Jc".to_string()),
    // &Card::from_name("Qc".to_string()),
    // &Card::from_name("Kc".to_string()),
    // &Card::from_name("Ac".to_string()),
    // &Card::from_name("2c".to_string()),
    // &Card::from_name("9c".to_string()),
    let rank = evaluate_7cards(
        Card::from_name(cards[0].to_string()).id_,
        Card::from_name(cards[1].to_string()).id_,
        Card::from_name(cards[2].to_string()).id_,
        Card::from_name(cards[3].to_string()).id_,
        Card::from_name(cards[4].to_string()).id_,
        Card::from_name(cards[5].to_string()).id_,
        Card::from_name(cards[6].to_string()).id_,
    );
    rank
}
fn display_all_cards() -> Vec<String> {
    let ranks = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    let suits = ['c', 'd', 'h', 's'];
    let mut cards = Vec::new();

    for (i, rank) in ranks.iter().enumerate() {
        for (j, suit) in suits.iter().enumerate() {
            let card_name = format!("{}{}", rank, suit);
            let card_number = i * 4 + j + 1;
            println!(
                "{}. {}",
                card_number,
                Card::from_name(card_name.clone()).to_clean_name()
            );
            cards.push(card_name);
        }
    }
    cards
}

fn get_card_by_number(card_number: usize, all_cards: &Vec<String>) -> String {
    if card_number > 0 && card_number <= all_cards.len() {
        all_cards[card_number - 1].clone()
    } else {
        panic!("Invalid card number");
    }
}

fn calculatePersonalHand() {
    // Input number of players
    let mut num_players_input = String::new();
    println!("Enter number of players:");
    io::stdin().read_line(&mut num_players_input).unwrap();
    let num_players: usize = match num_players_input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input for number of players.");
            return;
        }
    };

    // Input number of card packs
    let mut pack_count_input = String::new();
    println!("Enter number of card packs:");
    io::stdin().read_line(&mut pack_count_input).unwrap();
    let pack_count: usize = match pack_count_input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input for number of card packs.");
            return;
        }
    };

    // Display all cards with numbers
    println!("\nAvailable cards:");
    let all_cards = display_all_cards();

    // Input 2 cards
    let mut cards_input = String::new();
    println!("\nEnter your 2 cards as numbers (e.g., 1,2):");
    io::stdin().read_line(&mut cards_input).unwrap();
    let card_numbers: Vec<usize> = cards_input
        .trim()
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    if card_numbers.len() != 2 {
        println!(
            "Invalid input for cards. Please enter exactly 2 card numbers in the format (1,2)."
        );
        return;
    }

    let cards: Vec<String> = card_numbers
        .iter()
        .map(|&num| get_card_by_number(num, &all_cards))
        .collect();
    println!("Your hole cards: {:?}", cards);

    // Convert string cards to Card objects
    let hole_cards: Vec<Card> = cards.iter().map(|s| Card::from_name(s.clone())).collect();
    let community_cards: Vec<Card> = Vec::new();

    // Calculate and display probabilities for hole cards
    println!("\nProbabilities with hole cards:");
    let hand_probs =
        calculate_hand_probabilities(&hole_cards, &community_cards, num_players, pack_count);
    for (category, prob) in probability::get_ordered_probabilities(&hand_probs) {
        println!("{}: {:.2}%", category, prob);
    }

    let win_prob =
        calculate_win_probability(&hole_cards, &community_cards, num_players, pack_count);
    println!("Win probability: {:.2}%", win_prob);

    // Initialize middle cards vector
    let mut cards_middle: Vec<String> = Vec::new();
    let mut community_cards: Vec<Card> = Vec::new();

    // Ask for flop (3 cards)
    let mut flop_input = String::new();
    println!("\nEnter the flop (3 cards) as numbers (e.g., 3,4,5) or press Enter to skip:");
    io::stdin().read_line(&mut flop_input).unwrap();
    let flop_input = flop_input.trim();

    if !flop_input.is_empty() {
        let flop_numbers: Vec<usize> = flop_input
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        if flop_numbers.len() != 3 {
            println!("Invalid input for flop. Please enter exactly 3 card numbers in the format (3,4,5).");
            return;
        }

        let flop_cards: Vec<String> = flop_numbers
            .iter()
            .map(|&num| get_card_by_number(num, &all_cards))
            .collect();
        cards_middle.extend(flop_cards.clone());
        println!("Flop: {:?}", cards_middle);

        // Convert string cards to Card objects
        community_cards = flop_cards
            .iter()
            .map(|s| Card::from_name(s.clone()))
            .collect();

        // Calculate and display probabilities for flop
        println!("\nProbabilities with flop:");
        let hand_probs =
            calculate_hand_probabilities(&hole_cards, &community_cards, num_players, pack_count);
        for (category, prob) in probability::get_ordered_probabilities(&hand_probs) {
            println!("{}: {:.2}%", category, prob);
        }

        let win_prob =
            calculate_win_probability(&hole_cards, &community_cards, num_players, pack_count);
        println!("Win probability: {:.2}%", win_prob);
    }

    // Ask for turn (1 card)
    if cards_middle.len() == 3 {
        let mut turn_input = String::new();
        println!("\nEnter the turn (1 card) as a number (e.g., 6) or press Enter to skip:");
        io::stdin().read_line(&mut turn_input).unwrap();
        let turn_input = turn_input.trim();

        if !turn_input.is_empty() {
            let turn_number: usize = turn_input.parse().unwrap();
            let turn_card = get_card_by_number(turn_number, &all_cards);
            cards_middle.push(turn_card.clone());
            println!("Turn: {:?}", cards_middle);

            // Add turn card to community cards
            community_cards.push(Card::from_name(turn_card));

            // Calculate and display probabilities for turn
            println!("\nProbabilities with turn:");
            let hand_probs = calculate_hand_probabilities(
                &hole_cards,
                &community_cards,
                num_players,
                pack_count,
            );
            for (category, prob) in probability::get_ordered_probabilities(&hand_probs) {
                println!("{}: {:.2}%", category, prob);
            }

            let win_prob =
                calculate_win_probability(&hole_cards, &community_cards, num_players, pack_count);
            println!("Win probability: {:.2}%", win_prob);
        }
    }

    // Ask for river (1 card)
    if cards_middle.len() == 4 {
        let mut river_input = String::new();
        println!("\nEnter the river (1 card) as a number (e.g., 7) or press Enter to skip:");
        io::stdin().read_line(&mut river_input).unwrap();
        let river_input = river_input.trim();

        if !river_input.is_empty() {
            let river_number: usize = river_input.parse().unwrap();
            let river_card = get_card_by_number(river_number, &all_cards);
            cards_middle.push(river_card.clone());
            println!("River: {:?}", cards_middle);

            // Add river card to community cards
            community_cards.push(Card::from_name(river_card));

            // Calculate and display probabilities for river
            println!("\nProbabilities with river:");
            let hand_probs = calculate_hand_probabilities(
                &hole_cards,
                &community_cards,
                num_players,
                pack_count,
            );
            for (category, prob) in probability::get_ordered_probabilities(&hand_probs) {
                println!("{}: {:.2}%", category, prob);
            }

            let win_prob =
                calculate_win_probability(&hole_cards, &community_cards, num_players, pack_count);
            println!("Win probability: {:.2}%", win_prob);
        }
    }

    // Combine the cards into a single vector
    let mut all_cards: Vec<&str> = Vec::new();
    all_cards.extend(cards.iter().map(|s| s.as_str()));
    all_cards.extend(cards_middle.iter().map(|s| s.as_str()));

    println!("All cards: {:?}", all_cards);

    // Only evaluate if we have at least 5 cards (2 hole cards + 3 community cards)
    if all_cards.len() >= 5 {
        let rank = get_rank_of_7_perfect(all_cards);
        let rank_category = describe_rank_category(get_rank_category(rank));
        // Convert rank to percentile (7462 is total number of distinct hands)
        let percentile = ((7462 - rank) as f64 / 7462.0 * 100.0).round() as i32;
        println!(
            "Your hand {} is in the {}th percentile (rank {})",
            rank_category, percentile, rank
        );
    } else {
        println!("Not enough cards to evaluate a hand. Need at least 5 cards (2 hole cards + 3 community cards).");
    }
}
fn main() {
    // let mut game = Game::new(5, 1000.0);
    calculatePersonalHand();
}
