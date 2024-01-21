//card.h equivalent.
use phf::phf_map;

pub const RANK_MAP: phf::Map<char, i32> = phf_map! {
     '2' => 0 ,  '3' => 1 ,  '4'=> 2 , '5' => 3 ,
     '6' => 4 ,  '7' => 5 ,  '8'=> 6 , '9' => 7 ,
     'T' => 8 ,  'J' => 9 ,  'Q'=> 10 , 'K' => 11 ,  'A' => 12 ,
};

//Clubs, Diamonds, Hearts and Spades
pub const SUIT_MAP: phf::Map<char, i32> = phf_map! {
      'C'  => 0    ,   'D'  => 1    ,  'H'  => 2   , 'S'  => 3,
      'c'  => 0    ,   'd'  => 1    ,  'h'  => 2   , 's'  => 3,
};
#[derive(Clone)]
pub struct Card {
    pub id_: i32,
}

impl Card {
    pub fn from_id(id_: i32) -> Card {
        Card { id_ }
    }

    pub fn from_name(name: String) -> Card {
        if name.len() < 2 {
            todo!("Throw an exception here");
        }
        let result = match RANK_MAP.get(&name.chars().nth(0).unwrap()) {
            Some(a) => a,
            None => &499, // Just using this as an error code.
        };

        let result2 = match SUIT_MAP.get(&name.chars().nth(1).unwrap()) {
            Some(a) => a,
            None => &500,
        };

        Card {
            id_: result * 4 + result2,
        }
    }
    pub fn to_name(&self) -> String {
        let rank_index = self.id_ / 4;
        let suit_index = self.id_ % 4;

        let rank_char = match RANK_MAP.entries().find(|&(_, &v)| v == rank_index) {
            Some((&k, _)) => k,
            None => todo!("Handle invalid rank index here"),
        };

        let suit_char = match SUIT_MAP.entries().find(|&(_, &v)| v == suit_index) {
            Some((&k, _)) => k,
            None => todo!("Handle invalid suit index here"),
        };

        format!("{}{}", rank_char, suit_char)
    }
    pub fn to_clean_name(&self) -> String {
        let name: String = self.to_name();
        let mut p1: String = String::new();
        let mut p2: String = String::new();

        if let Some(c) = name.chars().nth(0) {
            p1.push(c);
        }

        if let Some(c) = name.chars().nth(1) {
            p2.push(c);
        }

        if p1 == "T" {
            p1 = "10".to_string();
        } else if p1 == "J" {
            p1 = "Jack".to_string();
        } else if p1 == "Q" {
            p1 = "Queen".to_string();
        } else if p1 == "K" {
            p1 = "King".to_string();
        } else if p1 == "A" {
            p1 = "Ace".to_string();
        }

        if p2 == "c" || p2 == "C" {
            p2 = "Clubs".to_string();
        } else if p2 == "d" || p2 == "D" {
            p2 = "Diamonds".to_string();
        } else if p2 == "h" || p2 == "H" {
            p2 = "Hearts".to_string();
        } else if p2 == "s" || p2 == "S" {
            p2 = "Spades".to_string();
        }

        p1 + " of " + &p2
    }

    pub fn int(&self) -> usize {
        self.id_ as usize
    }
    pub fn int32(&self) -> i32 {
        self.id_ as i32
    }
}
