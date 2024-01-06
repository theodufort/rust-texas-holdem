use crate::card::Card;

pub struct Player {
    pub cards: Vec<Card>,
}
impl Player {
    pub fn new() -> Player {
        Player { cards: Vec::new() }
    }
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}
