use crate::card::Card;
use std::fmt;
pub enum DecisionType {
    Fold,
    Check,
    Call,
    Pending,
}

impl Default for DecisionType {
    fn default() -> Self {
        Self::Pending
    }
}
impl fmt::Display for DecisionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecisionType::Fold => write!(f, "Fold"),
            DecisionType::Check => write!(f, "Check"),
            DecisionType::Call => write!(f, "Call"),
            DecisionType::Pending => write!(f, "Pending"),
        }
    }
}
#[derive(Default)]
pub struct Decision {
    pub decision_type: DecisionType,
    pub raise_amount: f32,
}
impl Decision {
    pub fn new(dt: DecisionType, ra: f32) -> Decision {
        Decision {
            decision_type: dt,
            raise_amount: ra,
        }
    }
}

pub struct Player {
    pub nickname: String,
    pub cards: Vec<Card>,
    pub is_folded: bool,
    pub balance: f32,
    pub wagered: f32,
    pub hand_rank: i32,
    pub decisions: Vec<Decision>,
}
impl Player {
    pub fn new() -> Player {
        Player {
            nickname: "Bing chilling".to_owned(),
            cards: Vec::new(),
            balance: 5000.0,
            wagered: 0.0,
            is_folded: false,
            hand_rank: 1,
            decisions: Vec::new(),
        }
    }
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
    pub fn check(&mut self, amount: f32) {
        if amount > self.balance {
            println!("Insufficient balance!");
        } else {
            self.increment_wager(amount);
            self.decrement_balance(amount);
        }
    }

    pub fn fold(&mut self) {
        self.is_folded = true;
    }

    pub fn wager(&mut self, amount: f32) {
        if amount > self.balance {
            println!("Insufficient balance!");
        } else {
            self.increment_wager(amount);
            self.decrement_balance(amount);
        }
    }

    pub fn increment_wager(&mut self, amount: f32) {
        self.wagered += amount;
    }

    pub fn decrement_balance(&mut self, amount: f32) {
        self.balance -= amount;
    }

    pub fn increment_balance(&mut self, amount: f32) {
        self.balance += amount;
    }
}
