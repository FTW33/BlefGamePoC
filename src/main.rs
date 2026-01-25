use crate::command::BetData;
use crate::players::Players;
use command::Command;
use config::Config;
use poker_combination::PokerCombination;
use std::io;

use crate::statics::*;

mod card_suit;
mod card_value;
mod command;
mod config;
mod hand;
mod players;
mod poker_combination;
mod statics;
mod utils;

fn main() {
    initial_greeting();
    // Most of this code should be in GameLogic
    let config: Config = players_config();
    play_game(
        Players::new(config.no_of_players),
        BetData::new(PokerCombination::None),
        config,
    );
    println!("Game over. A player reached the card limit. Press ENTER to continue");

    io::stdin()
        .read_line(&mut String::new())
        .expect("Failed in reading user input");
}

// To refactor and extract all the below functions to GameLogic module
fn play_game(mut players: Players, mut current_bet: BetData, config: Config) {
    println!("Beginning new round");
    println!("All cards: {:?}", players.get_all_cards());

    while !players.is_limit_hit(config.card_on_hand_limit) {
        println!("Current bet: {:?}", current_bet);
        match command::get_next_command() {
            Command::Bet(data) => {
                handle_new_bet(data, &mut current_bet);
            }
            Command::Call => {
                // TODO if combination is none repeat the turn
                players.handle_call(current_bet.poker_combination());
                reset_game_state(&mut players, &mut current_bet);
            }
        }
        utils::clear_screen();
        players.next_player();
    }
}

fn reset_game_state(players: &mut Players, current_bet: &mut BetData) {
    players.empty_all_cards();
    players.deal_cards();
    *current_bet = BetData::new(PokerCombination::None);
}

fn handle_new_bet(new_bet: BetData, current_bet: &mut BetData) {
    if new_bet <= *current_bet {
        println!("The new bet has to be bigger than the existing one");
    } else {
        *current_bet = new_bet;
    }
}
