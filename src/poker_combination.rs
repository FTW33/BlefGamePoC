use std::fmt::{Display, Formatter};

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum PokerCombination {
    None,
    HighCard,
    Pair,
    TwoPairs,
    Three,
    FullHouse,
    Quad,
    Straight,
    Flush,
    StraightFlush,
    RoyalFlush,
}

impl Display for PokerCombination {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let combination_string = match self {
            PokerCombination::None => "none",
            PokerCombination::HighCard => "highcard",
            PokerCombination::Pair => "pair",
            PokerCombination::TwoPairs => "twopairs",
            PokerCombination::Three => "three",
            PokerCombination::FullHouse => "fullhouse",
            PokerCombination::Quad => "quad",
            PokerCombination::Straight => "straight",
            PokerCombination::Flush => "flush",
            PokerCombination::StraightFlush => "straightflush",
            PokerCombination::RoyalFlush => "royalflush",
        };
        write!(f, "{}", combination_string)
    }
}

impl TryFrom<&str> for PokerCombination {
    // change into an actual error type
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().trim_end().to_lowercase().as_str() {
            "none" => Ok(PokerCombination::None),
            "highcard" => Ok(PokerCombination::HighCard),
            "pair" => Ok(PokerCombination::Pair),
            "twopairs" => Ok(PokerCombination::TwoPairs),
            "three" => Ok(PokerCombination::Three),
            "fullhouse" => Ok(PokerCombination::FullHouse),
            "quad" => Ok(PokerCombination::Quad),
            "straight" => Ok(PokerCombination::Straight),
            "flush" => Ok(PokerCombination::Flush),
            "straightflush" => Ok(PokerCombination::StraightFlush),
            "royalflush" => Ok(PokerCombination::RoyalFlush),
            _ => Err("Invalid PokerCombination name".to_string()),
        }
    }
}
