use std::fmt;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum CardType {
    Clover,
    Spades,
    Tile,
    Heart,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Copy)]
#[repr(u8)]
pub enum CardValue {
    Ace = 1,
    Number(u8),
    Jack = 11,
    Queen = 12,
    King = 13,
}
impl CardValue {
    // Create a new CardValue from a number
    fn new_number(value: u8) -> Option<CardValue> {
        if value >= 2 && value <= 10 {
            Some(CardValue::Number(value))
        } else {
            None
        }
    }
}

impl fmt::Display for CardValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardValue::Ace => write!(f, "Ace"),
            CardValue::Number(n) => write!(f, "{}", n),
            CardValue::Jack => write!(f, "Jack"),
            CardValue::Queen => write!(f, "Queen"),
            CardValue::King => write!(f, "King"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Card {
    pub card_type: CardType,
    pub card_value: CardValue,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of ({:?})", self.card_value, self.card_type)
    }
}

impl Card {
    // Constructor for a Card
    pub fn new(card_type: CardType, card_value: CardValue) -> Card {
        Card {
            card_type,
            card_value,
        }
    }
}
