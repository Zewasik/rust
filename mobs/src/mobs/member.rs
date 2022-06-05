pub mod member {
    pub enum Role {
        Underboss,
        Caporegime,
        Soldier,
        Associate,
    }

    pub struct Member {
        name: String,
        role: Role,
        age: u8,
    }

    impl Member {
        pub fn get_promotion(&self) {} // promote from -> to: Associate > Solder, Soldier -> Caporegime, Caporegime -> Underboss

        pub fn new(name: String, role: Role, age: u8) -> Member {
            Member { name, role, age }
        }
    }
}
