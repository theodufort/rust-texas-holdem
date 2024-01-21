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

// Export relevant types for external use
pub use card::Card;
pub use deck::Deck;
pub use evaluator::evaluate_cards;
use evaluator7::evaluate_7cards;
pub use game::Game;
pub use player::Player;
fn get_rank_of_7_perfect() {
    let rank = evaluate_cards(
        &Card::from_name("Tc".to_string()),
        &Card::from_name("Jc".to_string()),
        &Card::from_name("Qc".to_string()),
        &Card::from_name("Kc".to_string()),
        &Card::from_name("Ac".to_string()),
        &Card::from_name("2c".to_string()),
        &Card::from_name("9c".to_string()),
    );
    println!("{}", rank.value())
}
fn main() {
    // let mut game = Game::new(5, 1000.0);
    // game.play_turn();
    // game.play_turn();
    // game.play_turn();
    // game.play_turn();
    get_rank_of_7_perfect();
}
