use crate::card::Card;
use itertools::Itertools;
fn spot_rf(cards: Vec<Card>) -> Vec<Card> {
    let mut dangerous_cards: Vec<Card> = Vec::new();

    let grouped_by_suit = cards.iter().group_by(|&card| card.card_type);
    let grouped_by_suit_iter = grouped_by_suit.into_iter();
    for (_, cards_in_suit) in grouped_by_suit_iter {
        let cards_in_suit_vec: Vec<_> = cards_in_suit.cloned().collect(); // Convert to Vec

        if cards_in_suit_vec.len() > 2 {
            let mut count_flush = 0;
            for c in &cards_in_suit_vec {
                match c.card_value {
                    CardValue::Ace
                    | CardValue::King
                    | CardValue::Queen
                    | CardValue::Jack
                    | CardValue::Number(10) => count_flush += 1,
                    _ => (),
                }
            }
            if count_flush == 3 {
                dangerous_cards.extend(cards_in_suit_vec);
            }
        }
    }
    dangerous_cards
}

// //Check probabilities of other player having a dangerous hand
// fn check_community_card_danger(community_cards: Vec<Card>) {}
