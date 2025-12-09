#![allow(unused)]
/*
https://docs.rs/dialoguer/0.12.0/dialoguer/
https://docs.rs/console/latest/console/
*/
use console::style;
use std::sync::LazyLock;
use dialoguer::{Select, theme::ColorfulTheme};
use dialoguer::Input;
pub static GREETING: LazyLock<String> = LazyLock::new(|| {
    style("Welcome to bluff!").on_red().bold().to_string()
});

pub fn initial_greeting()
{
    println!("{}",*GREETING);
}