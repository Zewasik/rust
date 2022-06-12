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
    pub fn recruit(&self, name: String, age: u8) {
        // self.members.push(); // todo push Member
    }

    pub fn attack(&self, other: Mob) {} // todo: remove last member if less power, if draw attackers lost
                                        // if last member left -> winners takes cities and wealth

    pub fn steal(&self, other: Mob, to_steal: u32) {} // steal wealth from another Mob, if wealth of target Mob < to_steal, steal all

    pub fn conquer_city(mobs: Vec<Mob>, city: (String, u8)) {} // if another doeasnt have that city then add to cities list
}
