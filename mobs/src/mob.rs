pub mod mobs {
    mod boss;
    mod member;

    #[derive(Debug, CLone, PartialEq)]
    pub struct Mob {
        name: String,
        boss: Boss,
        members: Vec<Member>,
        cities: Vec<(String, u8)>,
        wealth: u32,
    }

    impl Mob {
        pub fn recruit(&self, name: String, age: u8) {
            self.members.push(); // todo push Member
        }

        pub fn attack(&self, other: Mob) {} // todo: remove last member if less power, if draw attackers lost
                                            // if last member left -> winners takes cities and wealth

        pub fn steal(&self, other: Mob, to_steal: u32) {} // steal wealth from another Mob, if wealth of target Mob < to_steal, steal all

        pub fn conquer_city(mobs: Vec<Mob>, city: (String, u8)) {} // if another doeasnt have that city then add to cities list
    }
}
