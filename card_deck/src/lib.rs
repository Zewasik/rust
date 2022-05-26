use rand::Rng;

#[derive(Debug, PartialEq, Eq)]

pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq, Eq)]

pub enum Rank {
    Ace,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack,
    Queen,
    King,
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        Suit::translate(rng.gen_range(1, 5))
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => return Suit::Heart,
            2 => return Suit::Diamond,
            3 => return Suit::Spade,
            4 => return Suit::Club,
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
            1 => return Rank::Ace,
            2 => return Rank::Two,
            3 => return Rank::Three,
            4 => return Rank::Four,
            5 => return Rank::Five,
            6 => return Rank::Six,
            7 => return Rank::Seven,
            8 => return Rank::Eight,
            9 => return Rank::Nine,
            10 => return Rank::Ten,
            11 => return Rank::Jack,
            12 => return Rank::Queen,
            13 => return Rank::King,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(car: &Card) -> bool {
    return car.suit == Suit::Spade && car.rank == Rank::Ace;
}
