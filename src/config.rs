use crate::statics::{LIMIT_CONFIG, PLAYERS_CONFIG};
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

pub struct Config {
    pub(crate) no_of_players: usize,
    pub(crate) card_on_hand_limit: u8,
}

impl Config {
    pub fn get_config() -> Self {
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
        Self {
            no_of_players,
            card_on_hand_limit,
        }
    }
}
