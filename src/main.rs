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
    id: u32,
    hand: Hand,
    chips: i32,
    blind: Blind,
    bet: i32,
    folded: bool,
}

impl Player {
    fn new() -> Self {
        return Self {
            id: 0,
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

struct Table {
    pot: i32,
    flop1: (Card, Suit),
    flop2: (Card, Suit),
    flop3: (Card, Suit),
    turn1: (Card, Suit),
    river1: (Card, Suit),
}

impl Table {
    fn new() -> Self {
        return Self {
            pot: 0,
            flop1: (Card::NONECARD, Suit::NONESUIT),
            flop2: (Card::NONECARD, Suit::NONESUIT),
            flop3: (Card::NONECARD, Suit::NONESUIT),
            turn1: (Card::NONECARD, Suit::NONESUIT),
            river1: (Card::NONECARD, Suit::NONESUIT),
        }
    }
}

fn print_game(players: &Vec<Player>, table: &Table) {
    for (i, player) in players.iter().enumerate() {
        println!("Player {}\nHand: {:?}{:?} {:?}{:?}\nChips: {}\nBet: {}\n", i, player.hand.card1.0, player.hand.card1.1, player.hand.card2.0, player.hand.card2.1, player.chips, player.bet);
    }
    println!("Table: {:?}{:?}, {:?}{:?}, {:?}{:?}, {:?}{:?}, {:?}{:?}\nPot: {}", 
        table.flop1.0, 
        table.flop1.1, 
        table.flop2.0, 
        table.flop2.1, 
        table.flop3.0, 
        table.flop3.1, 
        table.turn1.0, 
        table.turn1.1, 
        table.river1.0, 
        table.river1.1, 
        table.pot
    );
}

fn print_players(players: &Vec<Player>) {
    for player in players {
        println!("Player {}\nHand: {:?}{:?} {:?}{:?}\nChips: {}\nBet: {}\nFolded: {}", player.id, player.hand.card1.0, player.hand.card1.1, player.hand.card2.0, player.hand.card2.1, player.chips, player.bet, player.folded);
    }
}

fn print_table(table: &Table) {
    println!("Table: {:?}{:?}, {:?}{:?}, {:?}{:?}, {:?}{:?}, {:?}{:?}\nPot: {}", 
        table.flop1.0, 
        table.flop1.1, 
        table.flop2.0, 
        table.flop2.1, 
        table.flop3.0, 
        table.flop3.1, 
        table.turn1.0, 
        table.turn1.1, 
        table.river1.0, 
        table.river1.1, 
        table.pot
    );
}

fn betting_round(starting_player: &mut usize, players: &mut Vec<Player>,  table: &mut Table) -> bool {
    let mut i = 1;
    let mut in_players: Vec<Player> = players.clone();
    in_players.retain(|in_players| !in_players.folded);

    loop {
        let current_player: usize = (in_players.iter().position(|in_players| in_players.id == *starting_player as u32).unwrap() + i - 1) % in_players.len();

        println!("Player {} bet:", in_players[current_player].id);

        let mut bet = String::new();

        io::stdin()
            .read_line(&mut bet)
            .expect("Failed to read line");

        let bet: i32 = bet.trim().parse::<i32>().expect("Please type a number!");

        if bet == -1 {
            in_players[current_player].folded = true;
            in_players[current_player].bet = 0;
            players[in_players[current_player].id as usize] = in_players[current_player].clone();
            if in_players[current_player].id == *starting_player as u32 && in_players.len() > 2{
                *starting_player = in_players[(current_player + 1) % in_players.len()].id as usize;
            } else if in_players.len() <= 2 {
                for j in 0..in_players.len() {
                    in_players[j].bet = 0;
                    players[in_players[j].id as usize] = in_players[j].clone();
                }
                return true;
            }
            in_players.retain(|in_players| !in_players.folded);
            i -= 1;
        } else {
            table.pot += bet;
            in_players[current_player].bet += bet;
            in_players[current_player].chips -= bet;
        }

        /* Check if betting round is over */
        let bidding_over;
        let mut different_bets = false;
        for j in 0..in_players.len() {
            // TODO: fix this condition
            if in_players[j].bet != in_players[(j + 1) % in_players.len()].bet {
                different_bets = true;
            }
        }

        bidding_over = !different_bets && (i >= in_players.len()) || (in_players.len() == 1);

        if bidding_over {
            for j in 0..in_players.len() {
                in_players[j].bet = 0;
                players[in_players[j].id as usize] = in_players[j].clone();
            }
            return false;
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

    let mut table: Table = Table::new();

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
    
    for (i, player) in players.iter_mut().enumerate() {
        player.id = i as u32;
        player.chips = 500;
    }

    let mut small_blind_player = rand::thread_rng().gen_range(0..num_players);
    let mut big_blind_player = (small_blind_player + 1) % num_players;

    loop {

        loop {
            deck.shuffle(&mut thread_rng());
            
            /* Deal 2 cards to each player */
            for player in &mut players {
                player.hand.card1 = deck.pop().unwrap();
                player.hand.card2 = deck.pop().unwrap();
            }

            /* Set blinds */
            players[small_blind_player].blind = Blind::SMALL;
            players[big_blind_player].blind = Blind::BIG;
            
            /* Forced blind bets */
            players[small_blind_player].bet += small_blind;
            players[small_blind_player].chips -= small_blind;
            players[big_blind_player].bet += big_blind;
            players[big_blind_player].chips -= big_blind;
            table.pot += big_blind + small_blind;

            print_players(&mut players);

            let mut one_player_left : bool;

            /* Rest of first betting round */
            one_player_left = betting_round(&mut (big_blind_player + 1), &mut players, &mut table);
            if one_player_left { break; }

            /* Flop */
            deck.pop().unwrap(); // burn
            table.flop1 = deck.pop().unwrap();
            table.flop2 = deck.pop().unwrap();
            table.flop3 = deck.pop().unwrap();
            print_table(&table);

            /* Post-flop betting round */
            one_player_left = betting_round(&mut small_blind_player, &mut players,  &mut table);
            if one_player_left { break; }

            print_players(&players);

            /* Turn */
            deck.pop().unwrap(); // burn
            table.turn1 = deck.pop().unwrap();
            print_table(&table);

            /* Post-turn betting round */
            one_player_left = betting_round(&mut small_blind_player, &mut players,  &mut table);
            if one_player_left { break; }

            print_players(&players);

            /* River */
            deck.pop().unwrap(); // burn
            table.river1 = deck.pop().unwrap();
            print_table(&table);

            /* Final betting round */
            one_player_left = betting_round(&mut small_blind_player, &mut players, &mut table);
            if one_player_left { break; }

            print_players(&players);

            /* Evaluate hands. When deciding winner, fold all hands that aren't winners in Player structs*/
            todo!();
        }
        
        let mut winners: Vec<Player> = vec!();
        for player in &mut players {
            if !player.folded {
                winners.push(player.clone());
            }
        }

        let num_winners = winners.len();

        for winner in &mut winners {
            winner.chips += (table.pot as f32 / num_winners as f32) as i32;
            players[winner.id as usize] = winner.clone();
        }

        small_blind_player = (small_blind_player + 1) % players.len();
        big_blind_player = (big_blind_player + 1) % players.len();

        print_players(&players);
    }
}
