use core::num;
use std::io;
use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
enum Card {
    NONECARD,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE,
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
enum Suit {
    NONESUIT,
    CLUBS,
    DIAMONDS,
    HEARTS,
    SPADES,
}

#[derive(PartialEq, Clone)]
enum Blind {
    NONEBLIND,
    SMALL,
    BIG,
}

#[derive(Clone)]
struct Player {
    hand: Hand,
    chips: i32,
    blind: Blind,
    bet: i32,
    folded: bool,
}

impl Player {
    fn new() -> Self {
        return Self {
            hand: Hand {card1: (Card::NONECARD, Suit::NONESUIT), card2: (Card::NONECARD, Suit::NONESUIT)},
            chips: 0,
            blind: Blind::NONEBLIND,
            bet: 0,
            folded: false,
        }
    }
}

#[derive(Clone)]
struct Hand {
    card1: (Card, Suit),
    card2: (Card, Suit),
}

fn betting_round(starting_player: usize, num_players: usize, players: &mut Vec<Player>) {
    let mut i = 0;
    loop {
        let current_player = (i + starting_player) % num_players;

        println!("Player {} bet:", current_player);

        let mut bet = String::new();

        io::stdin()
            .read_line(&mut bet)
            .expect("Failed to read line");

        let bet: i32 = bet.trim().parse::<i32>().expect("Please type a number!");

        if bet == -1 {
            players[current_player].folded = true;
        } else {
            players[current_player].bet = bet;
        }

        /* Check if bidding round is over */
        let bidding_over;
        let mut different_bets = false;
        for j in 0..num_players {
            // TODO: fix this condition
            if (players[j].bet != players[(j + 1) % num_players].bet) && !players[j].folded {
                different_bets = true;
            }
        }
        bidding_over = !different_bets;

        if bidding_over {
            break;
        } 

        i += 1;
    }
}

fn main() {
    let mut deck: Vec<(Card, Suit)> = vec!(
        (Card::ACE, Suit::CLUBS),
        (Card::ACE, Suit::DIAMONDS),
        (Card::ACE, Suit::HEARTS),
        (Card::ACE, Suit::SPADES),
        (Card::TWO, Suit::CLUBS),
        (Card::TWO, Suit::DIAMONDS),
        (Card::TWO, Suit::HEARTS),
        (Card::TWO, Suit::SPADES),
        (Card::THREE, Suit::CLUBS),
        (Card::THREE, Suit::DIAMONDS),
        (Card::THREE, Suit::HEARTS),
        (Card::THREE, Suit::SPADES),
        (Card::FOUR, Suit::CLUBS),
        (Card::FOUR, Suit::DIAMONDS),
        (Card::FOUR, Suit::HEARTS),
        (Card::FOUR, Suit::SPADES),
        (Card::FIVE, Suit::CLUBS),
        (Card::FIVE, Suit::DIAMONDS),
        (Card::FIVE, Suit::HEARTS),
        (Card::FIVE, Suit::SPADES),
        (Card::SIX, Suit::CLUBS),
        (Card::SIX, Suit::DIAMONDS),
        (Card::SIX, Suit::HEARTS),
        (Card::SIX, Suit::SPADES),
        (Card::SEVEN, Suit::CLUBS),
        (Card::SEVEN, Suit::DIAMONDS),
        (Card::SEVEN, Suit::HEARTS),
        (Card::SEVEN, Suit::SPADES),
        (Card::EIGHT, Suit::CLUBS),
        (Card::EIGHT, Suit::DIAMONDS),
        (Card::EIGHT, Suit::HEARTS),
        (Card::EIGHT, Suit::SPADES),
        (Card::NINE, Suit::CLUBS),
        (Card::NINE, Suit::DIAMONDS),
        (Card::NINE, Suit::HEARTS),
        (Card::NINE, Suit::SPADES),
        (Card::TEN, Suit::CLUBS),
        (Card::TEN, Suit::DIAMONDS),
        (Card::TEN, Suit::HEARTS),
        (Card::TEN, Suit::SPADES),
        (Card::JACK, Suit::CLUBS),
        (Card::JACK, Suit::DIAMONDS),
        (Card::JACK, Suit::HEARTS),
        (Card::JACK, Suit::SPADES),
        (Card::QUEEN, Suit::CLUBS),
        (Card::QUEEN, Suit::DIAMONDS),
        (Card::QUEEN, Suit::HEARTS),
        (Card::QUEEN, Suit::SPADES),
        (Card::KING, Suit::CLUBS),
        (Card::KING, Suit::DIAMONDS),
        (Card::KING, Suit::HEARTS),
        (Card::KING, Suit::SPADES),
    );

    println!("Enter the number of players:");

    let mut num_players = String::new();

    io::stdin()
        .read_line(&mut num_players)
        .expect("Failed to read line");

    let num_players: usize = num_players.trim().parse().expect("Please type a number!");

    println!("Big blind:");

    let mut big_blind = String::new();

    io::stdin()
        .read_line(&mut big_blind)
        .expect("Failed to read line");

    let big_blind: i32 = big_blind.trim().parse::<i32>().expect("Please type a number!");

    println!("Small blind:");

    let mut small_blind = String::new();

    io::stdin()
        .read_line(&mut small_blind)
        .expect("Failed to read line");

    let small_blind: i32 = small_blind.trim().parse::<i32>().expect("Please type a number!");

    let mut players: Vec<Player> = vec!(Player::new(); num_players);

    deck.shuffle(&mut thread_rng());
    
    /* deal 2 cards to each player, initial chips */
    for (i, player) in players.iter_mut().enumerate() {
        player.chips = 500;
        player.hand.card1 = deck.pop().unwrap();
        player.hand.card2 = deck.pop().unwrap();

        println!("Player {}\nHand: {:?}{:?} {:?}{:?}\nChips: {}\n Bet: {}\n", i, player.hand.card1.0, player.hand.card1.1, player.hand.card2.0, player.hand.card2.1, player.chips, player.bet);
    }

    /* Decide blinds */
    let mut small_blind_player = rand::thread_rng().gen_range(0..num_players);
    let mut big_blind_player = (small_blind_player + 1) % num_players;
    players[small_blind_player].blind = Blind::SMALL;
    players[big_blind_player].blind = Blind::BIG;
    
    /* Forced blind bets */
    players[small_blind_player].bet += small_blind;
    players[small_blind_player].chips -= small_blind;
    players[big_blind_player].bet += big_blind;
    players[big_blind_player].chips -= big_blind;

    /* Go through rest of betting round */
    betting_round(big_blind_player + 1, num_players, &mut players);
    
    /* Flop */
    deck.pop().unwrap(); // burn
    let flop1 = deck.pop().unwrap();
    let flop2 = deck.pop().unwrap();
    let flop3 = deck.pop().unwrap();
    println!("{:?}{:?}, {:?}{:?}, {:?}{:?}", flop1.0, flop1.1, flop2.0, flop2.1, flop3.0, flop3.1);

    /* Post-flop betting round */
    betting_round(small_blind_player, num_players, &mut players);

    /* Turn */
    deck.pop().unwrap(); // burn
    let turn1 = deck.pop().unwrap();
    println!("Table: {:?}{:?}, {:?}{:?}, {:?}{:?}, {:?}{:?}", flop1.0, flop1.1, flop2.0, flop2.1, flop3.0, flop3.1, turn1.0, turn1.1);

    /* Post-turn betting round */
    betting_round(small_blind_player, num_players, &mut players);

    /* River */
    deck.pop().unwrap(); // burn
    let river1 = deck.pop().unwrap();
    println!("Table: {:?}{:?}, {:?}{:?}, {:?}{:?}, {:?}{:?}. {:?}{:?}", flop1.0, flop1.1, flop2.0, flop2.1, flop3.0, flop3.1, turn1.0, turn1.1, river1.0, river1.1);

    /* Final betting round */
    betting_round(small_blind_player, num_players, &mut players);

    /* Evaluate hands */
    todo!();
}
