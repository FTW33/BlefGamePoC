use commands::Commands;
use config::Config;
use poker_combination::PokerCombination;
use std::io;

use crate::players::{Player, Players};

mod card_suit;
mod card_value;
mod commands;
mod config;
mod hand;
mod players;
mod poker_combination;
mod utils;

fn main() {
    // Most of this code should be in GameLogic
    println!("Welcome to bluff!");
    let config: Config = Config::get_config();
    play_game(Players::new(config.no_of_players), PokerCombination::None, config);
    println!("Game over. A player reached the card limit. Press ENTER to continue");

    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed in reading user input");
}

// To refactor and extract all the below functions to GameLogic module
fn play_game(mut players: Players, mut current_bet: PokerCombination, config: Config) {
    println!("Beginning new round");
    println!("All cards: {:?}", players.get_all_cards());

    while !players.is_limit_hit(config.card_on_hand_limit) {
        println!("Current bet: {:?}", current_bet);
        // Move the below command getting loop into a method in commands? try_get_next_command_until_success?
        let mut command = commands::get_next_command();
        while command == Commands::Unknown {
            command = commands::get_next_command();
        }
        match command {
            Commands::Bet(value) => {
                handle_new_bet(value, &mut current_bet);
            }
            Commands::Call => {
                // TODO if combination is none repeat the turn
                players.handle_call(&current_bet);
                reset_game_state(&mut players, &mut current_bet);
            }
            Commands::Unknown => {}
        }
        utils::clear_screen();
        players.next_player();
    }
}

fn reset_game_state(players: &mut Players, current_bet: &mut PokerCombination) {
    players.empty_all_cards();
    players.deal_cards();
    *current_bet = PokerCombination::None;
}

fn handle_new_bet(new_bet: PokerCombination, current_bet: &mut PokerCombination) {
    if new_bet <= *current_bet {
        println!("The new bet has to be bigger than the existing one");
    } else {
        *current_bet = new_bet;
    }
}

fn get_bet(bet_str: &str) -> Result<PokerCombination, String> {
    PokerCombination::try_from(bet_str)
}
