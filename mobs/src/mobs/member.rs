#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Underboss,
    Caporegime,
    Soldier,
    Associate,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    name: String,
    role: Role,
    age: u8,
}

impl Member {
    pub fn get_promotion(&self) {} // promote from -> to: Associate > Solder, Soldier -> Caporegime, Caporegime -> Underboss

    pub fn new(name: &str, role: Role, age: u8) -> Member {
        Member {
            name: name.to_string(),
            role,
            age,
        }
    }
}
