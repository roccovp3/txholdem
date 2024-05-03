use std::io;

const CARDS: [char; 13] = ['A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K'];

#[derive(Clone, Default)]
struct Player {
    hand: Hand,
    chips: Chips,
}

impl Player {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Clone, Default)]
struct Hand {
    card1: char,
    card2: char,
}

impl Hand {
    fn new() -> Self {
        Default::default()
    }
}

// hard code denominations for now as (value, count)
#[derive(Clone, Default)]
struct Chips {
    white: (u32, u32),
    green: (u32, u32),
    blue: (u32, u32),
    red: (u32, u32),
    black: (u32, u32),
}

impl Chips {
    fn new() -> Self {
        Default::default()
    }
}

fn main() {
    println!("Enter the number of players:");

    let mut num_players = String::new();

    io::stdin()
        .read_line(&mut num_players)
        .expect("Failed to read line");

    let num_players: usize = num_players.trim().parse().expect("Failed to convert to integer");

    let players: Vec<Player> = vec!(Player::new(); num_players);

    for mut player in players {
        //deal 2 card to each player
        todo!();
        player.hand.card1 = 'A';
        println!("{}", i.hand.card1);
    }
}
