use crate::{
    card::Card,
    deck::Deck,
    evaluator7,
    player::{self, Decision, DecisionType, Player},
};
use itertools::MinMaxResult;
use std::{
    borrow::{Borrow, BorrowMut},
    fmt::format,
    io::{self, Write},
};
use term_size::dimensions;

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H"); // ANSI escape sequence for clearing the console
    io::stdout().flush().unwrap();
}

#[derive(Debug)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}
pub struct Game {
    pub player_count: u8,
    pub max_players: u8,
    pub players: Vec<Player>,
    pub deck: Deck,
    pub community_cards: Vec<Card>,
    pub turn_count: u8,
    pub pot_value: f32,
    pub entry_cost: f32,
}
impl Game {
    pub fn new(player_count: u8, entry_cost: f32) -> Game {
        let mut new_deck = Deck::new(4);
        Deck::shuffle_deck(&mut new_deck);
        let mut game = Game {
            player_count: player_count,
            max_players: 6,
            players: (0..player_count).map(|_| Player::new()).collect(),
            deck: new_deck,
            community_cards: Vec::new(),
            turn_count: 0,
            pot_value: 0.0,
            entry_cost: entry_cost,
        };
        game.distribute_cards_to_players();
        game
    }
    pub fn distribute_cards_to_players(&mut self) {
        // Ensure there are enough cards in the deck for all players
        if self.deck.cards.len() < self.player_count as usize * 2 {
            panic!("Not enough cards in the deck to distribute to players.");
        }

        // Distribute cards to each player
        for player in &mut self.players {
            for _ in 0..2 {
                if let Some(card) = self.deck.cards.pop() {
                    player.add_card(card);
                }
            }
        }
    }
    fn display_game_for_player(player: &Player) {
        print!("{}: ", player.nickname);
        for dec in &player.decisions {
            print!("{}, ", dec.decision_type);
        }
        print!("\n");
    }

    pub fn check_hand_rank(&self, cards: &Vec<Card>) {}
    fn release_card(&mut self) {
        // Ensure there are enough cards in the deck for all players
        if self.deck.cards.len() < self.player_count as usize * 2 {
            panic!("Not enough cards in the deck to distribute to players.");
        }

        if let Some(card) = self.deck.cards.pop() {
            self.community_cards.push(card);
        }
    }
    pub fn decision_round(&mut self, round_number: u8) {
        let mut minimum_wager_round = 0.0;
        for pl in &mut self.players {
            println!("Player {} must choose what he does: ", pl.nickname);
            print!("Player {} has the following cards: ", pl.nickname);
            //Print cards
            for card in &pl.cards {
                print!("{}, ", card.to_clean_name());
            }
            println!("\nCurrent minimum wager is {}", minimum_wager_round);
            println!("1) Fold");
            println!("2) Check");
            println!("3) Wager");
            loop {
                println!("Your choice: ");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                match input.trim() {
                    "1" => {
                        if round_number == 0 {
                            pl.decisions.push(Decision::new(DecisionType::Fold, 0.0));
                        } else if round_number == 1 {
                            pl.decisions.push(Decision::new(DecisionType::Fold, 0.0));
                        } else if round_number == 2 {
                            pl.decisions.push(Decision::new(DecisionType::Fold, 0.0));
                        } else if round_number == 3 {
                            pl.decisions.push(Decision::new(DecisionType::Fold, 0.0));
                        }
                        pl.fold();
                        break;
                    }
                    "2" => {
                        if round_number == 0 {
                            pl.decisions.push(Decision::new(DecisionType::Check, 0.0));
                        } else if round_number == 1 {
                            pl.decisions.push(Decision::new(DecisionType::Check, 0.0));
                        } else if round_number == 2 {
                            pl.decisions.push(Decision::new(DecisionType::Check, 0.0));
                        } else if round_number == 3 {
                            pl.decisions.push(Decision::new(DecisionType::Check, 0.0));
                        }
                        pl.check(minimum_wager_round);
                        self.pot_value += minimum_wager_round;
                        break;
                    }
                    "3" => {
                        let wager_amount: u32 = loop {
                            println!("Enter wager amount (1% to 100% of your balance): ");
                            let mut wager_input = String::new();
                            io::stdin()
                                .read_line(&mut wager_input)
                                .expect("Failed to read line");

                            match wager_input.trim().parse() {
                                Ok(amount) if (1..=100).contains(&amount) => break amount,
                                Ok(_) => {
                                    println!("Invalid wager amount. Must be between 1 and 100.")
                                }
                                Err(_) => println!("Invalid input. Please enter a number."),
                            }
                        };
                        let wager_value = pl.balance * wager_amount as f32 / 100.0;
                        //Set new minimum wager made by other player
                        if (minimum_wager_round < wager_value) {
                            minimum_wager_round = wager_value;
                        }
                        if round_number == 0 {
                            pl.decisions.push(Decision::new(DecisionType::Call, 0.0));
                        } else if round_number == 1 {
                            pl.decisions.push(Decision::new(DecisionType::Call, 0.0));
                        } else if round_number == 2 {
                            pl.decisions.push(Decision::new(DecisionType::Call, 0.0));
                        } else if round_number == 3 {
                            pl.decisions.push(Decision::new(DecisionType::Call, 0.0));
                        }
                        pl.wager(minimum_wager_round + wager_value);
                        self.pot_value += minimum_wager_round + wager_value;
                        break;
                    }
                    _ => println!("Invalid input. Please enter 1, 2, or 3."),
                }
            }
            clear_console();
        }
    }
    pub fn dynamic_pad_string(input: &str, padding_char: char) -> String {
        // Get terminal width
        let term_width = if let Some((w, h)) = dimensions() {
            w
        } else {
            // Default to a reasonable width if terminal dimensions can't be determined
            80
        };

        // Calculate available space for padding on both sides
        let available_space = term_width as usize - input.len();

        // Calculate left and right padding lengths
        let left_padding_len = available_space / 2;
        let right_padding_len = available_space - left_padding_len;

        // Create left and right padding strings
        let left_padding: String = (0..left_padding_len).map(|_| padding_char).collect();
        let right_padding: String = (0..right_padding_len).map(|_| padding_char).collect();

        // Concatenate the padded string
        format!("{}{}{}", left_padding, input, right_padding)
    }
    fn display_danger() {
        println!("=== Dangers from other player's hands based on community cards ===");
        println!("============");
    }
    fn display_game(&mut self) {
        clear_console();
        println!(
            "{}",
            Self::dynamic_pad_string("Poker Game: Texas Holdem", ' ')
        );
        println!(
            "{}",
            Self::dynamic_pad_string(&format!("Player Count: {}", self.player_count), ' ')
        );
        println!(
            "{}",
            Self::dynamic_pad_string(&format!("Pack Count: {}", self.deck.pack_count), ' ')
        );
        let mut comm_cards: String = "Community Cards: ".to_owned();
        for cc in &self.community_cards {
            comm_cards += &cc.to_clean_name();
            comm_cards += ", ";
        }
        print!("{}", Self::dynamic_pad_string(&comm_cards, ' '));

        println!("{}", Self::dynamic_pad_string("\nPlayers ", ' '));
        for pl in &mut self.players {
            Self::display_game_for_player(pl); // <-- Modified line
        }
        println!("");
    }
    pub fn play_turn(&mut self) {
        println!("turn #: {}", self.turn_count);
        if self.turn_count == 0 {
            self.display_game();
            self.decision_round(self.turn_count);
            self.display_game();
            // Release 3 cards
            self.release_card();
            self.release_card();
            self.release_card();
            self.display_game();
            self.decision_round(self.turn_count);
            self.display_game();
        } else if self.turn_count == 1 {
            self.release_card();
            self.display_game();
            self.decision_round(self.turn_count);
            self.display_game();
        } else if self.turn_count == 2 {
            self.release_card();
            self.display_game();
            self.decision_round(self.turn_count);
            self.display_game();
        } else if self.turn_count == 3 {
            for pl in &mut self.players {
                let mut combined_cards: Vec<Card> = Vec::new();
                combined_cards.extend(self.community_cards.clone()); // Clone the community cards
                combined_cards.extend(pl.cards.clone()); // Use a reference to avoid moving pl.cards

                let rank = evaluator7::evaluate_7cards(
                    combined_cards[0].int32(),
                    combined_cards[1].int32(),
                    combined_cards[2].int32(),
                    combined_cards[3].int32(),
                    combined_cards[4].int32(),
                    combined_cards[5].int32(),
                    combined_cards[6].int32(),
                );
                pl.hand_rank = rank;
            }
            //Endgame
            let min_player = self
                .players
                .iter()
                .min_by(|&a, &b| a.hand_rank.cmp(&b.hand_rank));
            if let Some(min_player) = self
                .players
                .iter()
                .min_by(|&a, &b| a.hand_rank.cmp(&b.hand_rank))
            {
                println!("Player {} won!", min_player.nickname);
                // Print other details if needed
            } else {
                println!("No players found");
            }
        }

        self.turn_count += 1;
    }
}
