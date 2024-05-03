use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(PartialEq, PartialOrd, Clone)]
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

#[derive(PartialEq, PartialOrd, Clone)]
enum Suit {
    NONESUIT,
    CLUBS,
    DIAMONDS,
    HEARTS,
    SPADES,
}

#[derive(Clone)]
struct Player {
    hand: Hand,
    chips: Chips,
}

impl Player {
    fn new() -> Self {
        return Self {
            hand: Hand {card1: (Card::NONECARD, Suit::NONESUIT), card2: (Card::NONECARD, Suit::NONESUIT)},
            chips: Chips {white: (0,0), green: (0,0), red: (0,0), blue: (0,0), black: (0,0)}
        }
    }
}

#[derive(Clone)]
struct Hand {
    card1: (Card, Suit),
    card2: (Card, Suit),
}

// hard code denominations for now as (value, count)
#[derive(Clone)]
struct Chips {
    white: (u32, u32),
    green: (u32, u32),
    blue: (u32, u32),
    red: (u32, u32),
    black: (u32, u32),
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

    let players: Vec<Player> = vec!(Player::new(); num_players);

    deck.shuffle(&mut thread_rng());

    for mut player in players {
        //deal 2 cards to each player, initial chips
        println!("{}", Suit::HEARTS > Suit::DIAMONDS);
        todo!();
    }
}
