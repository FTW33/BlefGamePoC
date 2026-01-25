use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(Clone, Copy, RandGen, Debug, Eq, PartialEq, Hash)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        };
        write!(f, "{}", s)
    }
}
