use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Suit(String),
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Rank(String),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        Suit::translate(rng.gen_range(1, 5))
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => return Suit::Suit("Heart".to_string()),
            2 => return Suit::Suit("Diamonds".to_string()),
            3 => return Suit::Suit("Spade".to_string()),
            4 => return Suit::Suit("Club".to_string()),
            _ => unreachable!(),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        Rank::translate(rng.gen_range(1, 14))
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            2..=10 => return Rank::Rank(value.to_string()),
            1 => return Rank::Rank("Ace".to_string()),
            11 => return Rank::Rank("Jack".to_string()),
            12 => return Rank::Rank("Queen".to_string()),
            13 => return Rank::Rank("King".to_string()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(car: &Card) -> bool {
    return car.suit == Suit::Suit("Spade".to_string())
        && car.rank == Rank::Rank("Ace".to_string());
}
