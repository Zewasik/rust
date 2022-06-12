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
    pub fn get_promotion(&mut self) {
        match self.role {
            Role::Associate => self.role = Role::Soldier,
            Role::Soldier => self.role = Role::Caporegime,
            Role::Caporegime => self.role = Role::Underboss,
            _ => (),
        }
    }

    pub fn new(name: String, role: Role, age: u8) -> Member {
        Member { name, role, age }
    }
}
