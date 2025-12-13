/*
https://docs.rs/dialoguer/0.12.0/dialoguer/
https://docs.rs/console/latest/console/
*/
use console::style;
use std::sync::LazyLock;
use dialoguer::{Input,Select, theme::ColorfulTheme};
pub static GREETING: LazyLock<String> = LazyLock::new(|| {
    style("Welcome to bluff!").on_red().bold().to_string()
});

pub fn initial_greeting()
{
    println!("{}",*GREETING);
}
pub static PLAYERS_CONFIG: LazyLock<[&str;5]> = LazyLock::new(|| {
     [
        "1", "2", "3", "4", "Other...",
     ]
});

pub fn players_config()
{
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select number of players to play")
        .default(0)
        .items(&PLAYERS_CONFIG[..])
        .interact()
        .unwrap();
    println!("Selection of {} players confirmed!", PLAYERS_CONFIG[selection]);
}
