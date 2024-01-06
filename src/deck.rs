use crate::card::{Card, CardType, CardValue};
use rand::seq::SliceRandom;
use std::fmt;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in &self.cards {
            writeln!(f, "{}", card)?;
        }
        Ok(())
    }
}

impl Deck {
    pub fn shuffle_deck(cards: &mut Vec<Card>) {
        let mut rng = rand::thread_rng();
        cards.shuffle(&mut rng);
    }
    pub fn new(pack_count: u8) -> Vec<Card> {
        let mut cards = Vec::new();

        for ct in &[
            CardType::Clover,
            CardType::Spades,
            CardType::Tile,
            CardType::Heart,
        ] {
            for cv in &[
                CardValue::Number(2),
                CardValue::Number(3),
                CardValue::Number(4),
                CardValue::Number(5),
                CardValue::Number(6),
                CardValue::Number(7),
                CardValue::Number(8),
                CardValue::Number(9),
                CardValue::Number(10),
                CardValue::Jack,
                CardValue::Queen,
                CardValue::King,
                CardValue::Ace,
            ] {
                if let CardValue::Number(n) = cv {
                    cards.push(Card::new(ct.clone(), CardValue::Number(n.to_owned())));
                } else {
                    cards.push(Card::new(ct.clone(), cv.clone()));
                }
            }
        }
        cards
    }
}
