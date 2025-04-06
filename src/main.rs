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
mod rank;
mod seven_four_six_two;

use std::io::{self, Read};

// Export relevant types for external use
pub use card::Card;
pub use deck::Deck;
use evaluator7::evaluate_7cards;
pub use game::Game;
pub use player::Player;
use rank::{describe_rank_category, get_rank_category};
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

    // Input 5 middle cards
    let mut cards_middle_input = String::new();
    println!("\nEnter the 5 cards in the middle as numbers (e.g., 1,2,3,4,5):");
    io::stdin().read_line(&mut cards_middle_input).unwrap();
    let middle_card_numbers: Vec<usize> = cards_middle_input
        .trim()
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    if middle_card_numbers.len() != 5 {
        println!("Invalid input for middle cards. Please enter exactly 5 card numbers in the format (1,2,3,4,5).");
        return;
    }

    let cards_middle: Vec<String> = middle_card_numbers
        .iter()
        .map(|&num| get_card_by_number(num, &all_cards))
        .collect();

    // Combine the cards into a single vector
    let mut all_cards: Vec<&str> = Vec::new();
    all_cards.extend(cards.iter().map(|s| s.as_str()));
    all_cards.extend(cards_middle.iter().map(|s| s.as_str()));

    println!("All cards: {:?}", all_cards);
    let rank = get_rank_of_7_perfect(all_cards);
    let rank_category = describe_rank_category(get_rank_category(rank));
    // Call the function with the combined cards
    println!("Your hand {} is in the {}", rank_category, rank);
}
fn main() {
    // let mut game = Game::new(5, 1000.0);
    calculatePersonalHand();
}
