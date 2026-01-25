/*
https://docs.rs/dialoguer/0.12.0/dialoguer/
https://docs.rs/console/latest/console/
*/
use crate::config::Config;
use console::style;
use dialoguer::{Select, theme::ColorfulTheme};
use std::sync::LazyLock;

pub static GREETING: LazyLock<String> =
    LazyLock::new(|| style("Welcome to bluff!").on_red().bold().to_string());

pub fn initial_greeting() {
    println!("{}", *GREETING);
}
pub static PLAYERS_CONFIG: LazyLock<[usize; 4]> = LazyLock::new(|| [1, 2, 3, 4]);

pub static LIMIT_CONFIG: LazyLock<[u8; 10]> = LazyLock::new(|| [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

pub fn players_config() -> Config {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select number of players to play")
        .default(0)
        .items(&PLAYERS_CONFIG[..])
        .interact()
        .unwrap();
    let no_of_players = PLAYERS_CONFIG[selection];
    println!("Selection of {} players confirmed!", no_of_players);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the card limit")
        .default(0)
        .items(&LIMIT_CONFIG[..])
        .interact()
        .unwrap();
    let card_on_hand_limit = LIMIT_CONFIG[selection];
    println!("Selection of limit={} confirmed!", card_on_hand_limit);
    Config {
        no_of_players,
        card_on_hand_limit,
    }
}
