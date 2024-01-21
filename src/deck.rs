use rand::seq::SliceRandom;

use crate::card::{Card, RANK_MAP, SUIT_MAP};

pub struct Deck {
    pub cards: Vec<Card>,
    pub pack_count: u8,
}

impl Deck {
    pub fn shuffle_deck(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
    pub fn new(pack_count: u8) -> Deck {
        let mut cards = Vec::new();
        RANK_MAP.into_iter().for_each(|rank| {
            SUIT_MAP.into_iter().for_each(|suit| {
                let new_card = rank.0.to_string() + &suit.0.to_string();
                cards.push(Card::from_name(new_card));
            });
        });
        Deck {
            cards: cards,
            pack_count: pack_count,
        }
    }
}
