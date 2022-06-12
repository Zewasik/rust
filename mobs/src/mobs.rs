pub mod boss;
pub mod member;

use crate::Boss;
use crate::Member;

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members
            .push(member::new(name, member::Role::Associate, age));
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let (mut loser, mut winner) = if &self.wealth > &other.wealth || self.members.len() != 0 {
            (other, self)
        } else {
            (self, other)
        };

        loser.members.pop();

        if loser.members.len() == 0 {
            winner.cities.append(&mut loser.cities);
            winner.wealth += loser.wealth;
            loser.wealth = 0;
        }
    }

    pub fn steal(&mut self, other: &mut Mob, to_steal: u32) {
        if to_steal > other.wealth {
            self.wealth += other.wealth;
            other.wealth = 0;
        } else {
            self.wealth += to_steal;
            other.wealth -= to_steal;
        }
    }

    pub fn conquer_city(&mut self, mobs: Vec<Mob>, name: String, value: u8) {
        let mut owned = false;

        for mob in mobs {
            for (find, _) in &mob.cities {
                if *find == name && *self != mob {
                    owned = true;
                    break;
                }
            }
        }

        if !owned {
            self.cities.push((name, value))
        }
    }
}
