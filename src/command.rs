use crate::card_suit::Suit;
use crate::card_suit::Suit::*;
use crate::card_value::CardValue;
use crate::card_value::CardValue::*;
use crate::command::Command::{Bet, Call};
use crate::poker_combination::PokerCombination;
use crate::poker_combination::PokerCombination::*;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use std::cmp::Ordering;
use std::fmt::Display;
use std::sync::LazyLock;

#[derive(PartialEq, Debug)]
pub enum Command {
    Bet(BetData),
    Call,
}
impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Bet(_) => "Bet",
            Call => "Call",
        };
        write!(f, "{}", s)
    }
}
static COMMAND: LazyLock<[Command; 2]> = LazyLock::new(|| {
    [
        Bet(BetData {
            poker_combination: None,
            suit: Option::None,
            value1: Option::None,
            value2: Option::None,
        }),
        Call,
    ]
});
static BET_COMBINATION: LazyLock<[PokerCombination; 10]> = LazyLock::new(|| {
    [
        HighCard,
        Pair,
        TwoPairs,
        PokerCombination::Three,
        FullHouse,
        Quad,
        Straight,
        Flush,
        StraightFlush,
        RoyalFlush,
    ]
});
static BET_SUIT: LazyLock<[Suit; 4]> = LazyLock::new(|| [Diamonds, Clubs, Hearts, Spades]);

static BET_VALUE: LazyLock<[CardValue; 13]> = LazyLock::new(|| {
    [
        Two,
        CardValue::Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    ]
});
#[derive(PartialEq, Debug)]
pub struct BetData {
    poker_combination: PokerCombination,
    suit: Option<Suit>,
    value1: Option<CardValue>,
    value2: Option<CardValue>,
}

impl BetData {
    pub fn new(poker_combination: PokerCombination) -> Self {
        BetData {
            poker_combination,
            suit: Option::None,
            value1: Option::None,
            value2: Option::None,
        }
    }
    pub fn with_suit(&mut self, suit: Suit) {
        self.suit = Some(suit);
    }
    pub fn with_value1(&mut self, value: CardValue) {
        self.value1 = Some(value);
    }
    pub fn with_value2(&mut self, value: CardValue) {
        self.value2 = Some(value);
    }

    pub fn poker_combination(&self) -> &PokerCombination {
        &self.poker_combination
    }
}

impl PartialOrd for BetData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.poker_combination != other.poker_combination
        {
            return self.poker_combination.partial_cmp(&other.poker_combination);
        }
        if self.value1.is_some() && other.value1.is_some() {
            if self.value1 != other.value1 {
                return self.value1.partial_cmp(&other.value1);
            }
            if self.value1.is_some() && other.value1.is_some() && self.value2 != other.value2 {
                return self.value2.partial_cmp(&other.value2);
            }
        }
        Some(Ordering::Equal)
    }
}

pub fn get_next_command() -> Command {
    let command_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select action")
        .default(0)
        .items(&COMMAND[..])
        .interact()
        .unwrap();
    match &COMMAND[command_selection] {
        Call => Call,
        Bet(_) => {
            let combination_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select figure")
                .default(0)
                .items(&BET_COMBINATION[..])
                .interact()
                .unwrap();

            let combination = BET_COMBINATION[combination_selection];
            let mut bet_data = BetData::new(combination);
            match combination {
                None => {
                    panic!("The combination cannot be None")
                }
                HighCard | Pair | PokerCombination::Three | Quad => {
                    let value = select_value_with_prompt("Select value for the figure");
                    bet_data.with_value1(value);
                    Bet(bet_data)
                }
                TwoPairs | FullHouse | Straight => {
                    let value1 = select_value_with_prompt("Select the first value for the figure");
                    let value2 = select_value_with_prompt("Select the second value for the figure");
                    bet_data.with_value1(value1);
                    bet_data.with_value2(value2);
                    Bet(bet_data)
                }
                Flush | StraightFlush | RoyalFlush => {
                    let suit_selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Select suit")
                        .default(0)
                        .items(&BET_SUIT[..])
                        .interact()
                        .unwrap();
                    let suit = BET_SUIT[suit_selection];
                    bet_data.with_suit(suit);
                    Bet(bet_data)
                }
            }
        }
    }
}

fn select_value_with_prompt(prompt: &str) -> CardValue {
    let value_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&BET_VALUE[..])
        .interact()
        .unwrap();
    BET_VALUE[value_selection]
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn command_correctly_parses_valid_bet_command() {
//         let valid_bet_command = "bet pair";
//         assert_eq!(
//             Ok(Command::Bet(PokerCombination::Pair)),
//             Command::from_str(valid_bet_command)
//         );
//     }
//     #[test]
//     fn command_fails_to_parse_invalid_bet_command() {
//         let invalid_bet_command = "bet";
//         assert!(Command::from_str(invalid_bet_command).is_err());
//
//         let invalid_bet_command = "bet pa";
//         assert!(Command::from_str(invalid_bet_command).is_err());
//     }
//
//     #[test]
//     fn command_fails_to_parse_invalid_command() {
//         let invalid_bet_command = "make coffee";
//         assert!(Command::from_str(invalid_bet_command).is_err());
//     }
//
//     #[test]
//     fn command_correctly_parses_valid_call_command() {
//         let valid_call_command = "call";
//         assert_eq!(Ok(Command::Call), Command::from_str(valid_call_command));
//     }
//
//     #[test]
//     fn command_fails_to_parse_invalid_call_command() {
//         let invalid_call_command = "call something";
//         assert!(Command::from_str(invalid_call_command).is_err());
//     }
// }
