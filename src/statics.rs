/*
https://docs.rs/dialoguer/0.12.0/dialoguer/
https://docs.rs/console/latest/console/
*/
use console::style;
use std::sync::LazyLock;

pub static GREETING: LazyLock<String> =
    LazyLock::new(|| style("Welcome to bluff!").on_red().bold().to_string());

pub fn initial_greeting() {
    println!("{}", *GREETING);
}
pub static PLAYERS_CONFIG: LazyLock<[usize; 4]> = LazyLock::new(|| [1, 2, 3, 4]);

pub static LIMIT_CONFIG: LazyLock<[u8; 10]> = LazyLock::new(|| [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
