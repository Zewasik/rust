#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A = 2,
    AB = 4,
    B = 3,
    O = 1,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
// #[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

// use std::cmp::{Ord, Ordering};

use std::{io::Error, str::FromStr};

impl FromStr for Antigen {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("AB") {
            Ok(Self::AB)
        } else if s.contains("A") {
            Ok(Self::A)
        } else if s.contains("B") {
            Ok(Self::B)
        } else if s.contains("O") {
            Ok(Self::O)
        } else {
            Ok(Self::O)
        }
    }
}

impl FromStr for RhFactor {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("AB") {
            Ok(Self::Positive)
        } else if s.contains("A") {
            Ok(Self::Negative)
        } else {
            Ok(Self::Positive)
        }
    }
}

// impl Ord for BloodType {}

impl FromStr for BloodType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let anti = s.parse().unwrap();
        let rh = s.parse().unwrap();

        Ok(BloodType {
            antigen: anti,
            rh_factor: rh,
        })
    }
}

// use std::fmt::{self, Debug};

// impl Debug for BloodType {}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        (self.rh_factor == other.rh_factor || self.rh_factor == RhFactor::Positive)
            && (self.antigen == other.antigen
                || self.antigen == Antigen::AB
                || other.antigen == Antigen::O)
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut v = vec![
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
        ];

        v.retain(|a| self.can_receive_from(a));
        v
    }

    pub fn recipients(&self) -> Vec<Self> {
        let mut v = vec![
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
        ];

        v.retain(|a| a.can_receive_from(self));
        v
    }
}
