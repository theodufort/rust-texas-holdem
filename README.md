# Rust Texas Hold'em Poker Probability Calculator

A sophisticated Texas Hold'em poker probability calculator implemented in Rust that calculates hand probabilities and winning odds in real-time.

## Features

- Calculate probabilities for all possible poker hand categories
- Compute winning probability against multiple opponents
- Support for multiple card decks
- Real-time probability updates at each stage (pre-flop, flop, turn, river)
- Hand strength percentile ranking
- Monte Carlo simulation for accurate probability estimation

## How It Works

### Core Components

1. **Card Representation (`card.rs`)**

   - Cards are represented with a unique ID (0-51)
   - Each card has a rank (2-A) and suit (clubs, diamonds, hearts, spades)
   - Conversion methods between string representation ("Ah" for Ace of hearts) and internal ID

2. **Hand Evaluation (`evaluator7.rs`)**

   - Uses a 7-card evaluator to determine hand strength
   - Returns a rank between 1 and 7462 (1 being the best possible hand)
   - Ranks are converted to standard poker hand categories (Straight Flush, Four of a Kind, etc.)

3. **Probability Calculation (`probability.rs`)**
   - Uses Monte Carlo simulation with 10,000 iterations
   - Two main probability calculations:
     - Hand category probabilities
     - Winning probabilities against opponents

### Probability Calculation Process

#### Hand Category Probabilities

1. Creates a deck excluding known cards (hole cards + community cards)
2. For each simulation:
   - Shuffles remaining deck
   - Completes community cards to 5 if needed
   - Evaluates resulting 7-card hand
   - Records hand category
3. Converts counts to percentages

#### Win Probability Calculation

1. Creates a deck excluding known cards
2. For each simulation:
   - Shuffles deck
   - Deals random hole cards to opponents
   - Completes community cards
   - Evaluates all hands
   - Compares player's hand against opponents
3. Calculates win percentage

### Hand Rankings

Hands are ranked in standard poker order (from highest to lowest):

1. Royal Flush (special case of Straight Flush)
2. Straight Flush
3. Four of a Kind
4. Full House
5. Flush
6. Straight
7. Three of a Kind
8. Two Pair
9. One Pair
10. High Card

### Hand Strength Percentile

- Converts raw rank (1-7462) to percentile (0-100%)
- Formula: ((7462 - rank) / 7462) \* 100
- Higher percentile indicates stronger hand
- Example: Rank 1 = 100th percentile, Rank 7462 = 0th percentile

## Usage

1. Run the program
2. Enter number of players
3. Enter number of card decks
4. Input hole cards using card numbers (displayed on screen)
5. Optionally input community cards as they appear:
   - Flop (3 cards)
   - Turn (1 card)
   - River (1 card)

The program will display:

- Current hand probabilities for each category
- Win probability against specified number of opponents
- Hand strength percentile when 5 or more cards are available

## Technical Details

### Monte Carlo Simulation

- Default 10,000 iterations for balance of accuracy and speed
- Configurable through `NUM_SIMULATIONS` constant
- Higher iterations increase accuracy but decrease performance

### Card Deck Management

- Supports multiple decks
- Automatically removes dealt cards from deck
- Prevents duplicate cards in simulations

### Performance Optimizations

- Efficient card representation using integer IDs
- Fast hand evaluation using pre-computed lookup tables
- Memory-efficient data structures
- Parallel simulation capability (future enhancement)

## Implementation Notes

### Key Data Structures

- `Card`: Represents a playing card with an ID
- `RankCategory`: Enum of poker hand categories
- `HashMap<RankCategory, f64>`: Stores category probabilities

### Important Functions

- `calculate_hand_probabilities`: Computes probabilities for each hand category
- `calculate_win_probability`: Determines chances of winning against opponents
- `get_ordered_probabilities`: Returns probabilities in standard poker hand order
- `evaluate_7cards`: Evaluates strength of a 7-card poker hand

### Error Handling

- Input validation for card numbers
- Deck overflow prevention
- Invalid hand detection
- Proper error messages for user guidance

## Future Enhancements

- Parallel processing for faster simulations
- GUI interface
- Hand history tracking
- Opponent modeling
- Pot odds calculator
- Tournament mode
