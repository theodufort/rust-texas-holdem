// You may need to use the #[path] attribute if the modules are not in the same directory.
#[path = "game.rs"]
mod game;

#[path = "card.rs"]
mod card;

#[path = "deck.rs"]
mod deck;

#[path = "player.rs"]
mod player;
fn main() {
    let mut _game = game::Game::new(5);
    _game.play_turn();
    _game.play_turn();
    _game.play_turn();
    _game.play_turn();
    _game.play_turn();
    for player in &_game.players {
        let mut concat_cards = player.cards.clone();
        concat_cards.extend(_game.community_cards.clone());
        println!(
            "{:?} {:?}",
            _game.check_hand_rank(&concat_cards),
            concat_cards
        );
    }
}
