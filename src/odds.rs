fn return_better_cards_possible(card_value: u8) -> u8 {
    14 - card_value
}
// Function to calculate combinations (n choose k)
pub fn combinations(n: u64, k: u64) -> f64 {
    if k > n {
        return 0.0;
    }

    let mut result = 1.0;
    for i in 0..k {
        result *= (n - i) as f64;
        result /= (i + 1) as f64;
    }

    result
}
fn calculate_poker(
    players: usize,
    community_cards: usize,
    hand: PokerHand,
) -> HashMap<PokerRank, f64> {
    let total_cards = 52 - players as u8 * 2 - community_cards as u8;

    let mut probabilities = HashMap::new();

    for rank in &[
        PokerRank::HighCard,
        PokerRank::OnePair,
        PokerRank::TwoPair,
        PokerRank::ThreeOfAKind,
        PokerRank::Straight,
        PokerRank::Flush,
        PokerRank::FullHouse,
        PokerRank::FourOfAKind,
        PokerRank::StraightFlush,
        PokerRank::RoyalFlush,
    ] {
        let rank = match rank {
            PokerRank::HighCard => 1.0,
            PokerRank::OnePair => calculate_one_pair(total_cards, 2),
            PokerRank::TwoPair => calculate_two_pair(total_cards, 2),
            PokerRank::ThreeOfAKind => calculate_three_of_a_kind(total_cards, 2),
            PokerRank::Straight => calculate_straight(total_cards, 2),
            PokerRank::Flush => calculate_flush(total_cards, 2),
            PokerRank::FullHouse => calculate_full_house(total_cards, 2),
            PokerRank::FourOfAKind => calculate_four_of_a_kind(total_cards, 2),
            PokerRank::StraightFlush => calculate_straight_flush(total_cards, 2),
            PokerRank::RoyalFlush => calculate_royal_flush(total_cards, 2),
        };

        probabilities.insert(*rank, rank);
    }

    probabilities
}

pub fn calculate_one_pair(pack_count: u8) -> f64 {
    (combinations(13, 6) - 71.0) * 6.0 * 6.0 * 990.0 / (133784560.0 * deck_count as f64) * 100.0
}

pub fn calculate_two_pairs(pack_count: u8) -> f64 {
    ((1277.0 * 10.0 * (6.0 * 62.0 + 24.0 * 63.0 + 6.0 * 64.0))
        + (combinations(13, 3) * combinations(4, 2).powf(3.0) * combinations(40, 1)))
        / (133784560.0 * deck_count as f64)
        * 100.0
}

fn calculate_three_of_a_kind(player_count: u8, cards: Vec<Card>, pack_count: u8) -> f64 {
    ((combinations(13, 5) - 10.0)
        * combinations(5, 1)
        * combinations(4, 1)
        * (combinations(4, 1).powf(4.0) - 3.0))
        / (133784560.0 * deck_count as f64)
        * 100.0
}

fn calculate_straight(player_count: u8, cards: Vec<Card>, pack_count: u8) -> f64 {
    ((217.0 * (4.0_f64.powf(7.0) - 756.0 - 4.0 - 84.0))
        + (71.0 * 36.0 * 990.0)
        + (10.0 * 5.0 * 4.0 * (256.0 - 3.0) + 10.0 * combinations(5, 2) * 2268.0))
        / (133784560.0 * deck_count as f64)
        * 100.0
}

fn calculate_flush(player_count: u8, cards: Vec<Card>, pack_count: u8) -> f64 {
    ((combinations(4, 1) * (combinations(13, 7) - 217.0))
        + (combinations(4, 1) * (combinations(13, 6) - 71.0) * 39.0)
        + (combinations(4, 1) * (combinations(13, 5) - 10.0) * combinations(39, 2)))
        / (133784560.0 * deck_count as f64)
        * 100.0
}

fn calculate_full_house(player_count: u8, cards: Vec<Card>, pack_count: u8) -> f64 {
    ((combinations(13, 2) * combinations(4, 3).powf(2.0) * combinations(44, 1))
        + (combinations(13, 1)
            * combinations(12, 2)
            * combinations(4, 3)
            * combinations(4, 2).powf(2.0))
        + (combinations(13, 1)
            * combinations(12, 1)
            * combinations(11, 2)
            * combinations(4, 3)
            * combinations(4, 2)
            * combinations(4, 1).powf(2.0)))
        / (133784560.0 * deck_count as f64)
        * 100.0
}

fn calculate_four_of_a_kind(player_count: u8, cards: Vec<Card>, pack_count: u8) -> f64 {
    combinations(13, 1) * combinations(48, 3) / (133784560.0 * deck_count as f64) * 100.0
}

fn calculate_straight_flush(player_count: u8, cards: Vec<Card>, pack_count: u8) -> f64 {
    combinations(9, 1) * combinations(4, 1) * combinations(46, 2)
        / (133784560.0 * deck_count as f64)
        * 100.0
}

fn calculate_royal_flush(player_count: u8, cards: Vec<Card>, pack_count: u8) -> f64 {
    combinations(4, 1) * combinations(47, 2) / (133784560.0 * deck_count as f64) * 100.0
}
