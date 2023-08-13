use colored::Colorize;
use itertools::{iproduct, Itertools};
//use rand;
use rand::prelude::SliceRandom;
use std::fmt;
use std::vec::Vec;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Property {
    First,
    Second,
    Third,
}

impl Property {
    fn third(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::First, Self::First) => Self::First,
            (Self::Second, Self::Second) => Self::Second,
            (Self::Third, Self::Third) => Self::Third,
            (Self::First, Self::Second) | (Self::Second, Self::First) => Self::Third,
            (Self::First, Self::Third) | (Self::Third, Self::First) => Self::Second,
            (Self::Second, Self::Third) | (Self::Third, Self::Second) => Self::First,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Card {
    shape: Property,
    shading: Property,
    color: Property,
    count: Property,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match (&self.shape, &self.shading) {
            // diamond, solid
            (Property::First, Property::First) => "■",
            // diamond, striped
            (Property::First, Property::Second) => "▥",
            // diamond, open
            (Property::First, Property::Third) => "□",

            // squiggle, solid
            (Property::Second, Property::First) => "▲",
            // squigle, striped
            (Property::Second, Property::Second) => "◬",
            // squiggle, open
            (Property::Second, Property::Third) => "△",

            // oval, solid
            (Property::Third, Property::First) => "●",
            // oval, striped
            (Property::Third, Property::Second) => "◍",
            // oval, open
            (Property::Third, Property::Third) => "○",
        };

        let number = match &self.count {
            Property::First => 1,
            Property::Second => 2,
            Property::Third => 3,
        };

        let repr = repr.repeat(number);
        let repr = match &self.color {
            Property::First => repr.red(),
            Property::Second => repr.green(),
            Property::Third => repr.purple(),
        };
        write!(f, "[{: <3}]", repr)
    }
}

impl Card {
    fn third(&self, other: &Card) -> Card {
        Card {
            shape: self.shape.third(&other.shape),
            shading: self.shading.third(&other.shading),
            color: self.color.third(&other.color),
            count: self.count.third(&other.count),
        }
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let mut deck: Vec<Card> = Vec::new();
        const VARIANTS: [Property; 3] = [Property::First, Property::Second, Property::Third];
        for (shape, shading, color, count) in iproduct!(
            VARIANTS.iter(),
            VARIANTS.iter(),
            VARIANTS.iter(),
            VARIANTS.iter()
        ) {
            deck.push(Card {
                shape: shape.clone(),
                shading: shading.clone(),
                color: color.clone(),
                count: count.clone(),
            });
        }

        let mut rng = rand::thread_rng();
        deck.shuffle(&mut rng);

        Self { cards: deck }
    }

    fn deal(&mut self, amount: usize) -> Vec<Card> {
        let amount = amount.min(self.cards.len());
        self.cards.split_off(self.cards.len() - amount)
    }
}

pub struct Game {
    deck: Deck,
    board: Vec<Card>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        let board = deck.deal(12);
        Self { deck, board }
    }

    pub fn print_board(&self) {
        for (i, x) in self.board.iter().enumerate() {
            print!("{}", x);
            if ((i + 1) % 4) == 0 {
                println!();
            }
        }
    }

    pub fn sets(&self) -> impl Iterator<Item = Vec<&Card>> {
        self.board
            .iter()
            .combinations(3)
            .filter(|v| v[0].third(v[1]) == *v[2])
    }
}
