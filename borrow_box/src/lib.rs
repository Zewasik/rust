#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Game {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nbr_of_games: u16,
}

impl Game {
    pub fn new(i: u32, pl1: String, pl2: String, n: u16) -> Box<Game> {
        Box::new(Game {
            id: i,
            p1: (pl1, 0),
            p2: (pl2, 0),
            nbr_of_games: n,
        })
    }

    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 == self.p2.1 {
            ("Same score! tied".to_string(), self.p1.1)
        } else if self.p1.1 > self.p2.1 {
            self.p1.clone()
        } else {
            self.p2.clone()
        }
    }
    pub fn update_score(&mut self, user_name: String) {
        if self.nbr_of_games > 0 && self.p1.1 < 3 && self.p2.1 < 3 {
            if self.p1.0 == user_name {
                self.p1.1 += 1;
                self.nbr_of_games -= 1;
            } else if self.p2.0 == user_name {
                self.p2.1 += 1;
                self.nbr_of_games -= 1;
            }
        }
    }

    pub fn delete(self) -> String {
        let id = self.id;
        drop(self);

        format!("game deleted: id -> {}", id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrow_box_test() {
        let mut game = Game::new(0, String::from("Joao"), String::from("Susana"), 5);

        assert_eq!(("Same score! tied".to_string(), 0), game.read_winner());

        game.update_score(String::from("Joao"));
        game.update_score(String::from("Joao"));
        game.update_score(String::from("Susana"));
        game.update_score(String::from("Susana"));

        assert_eq!(("Same score! tied".to_string(), 2), game.read_winner());

        game.update_score(String::from("Joao"));
        game.update_score(String::from("Susana")); // this one will not count because it already 5 games played, the nbr_of_games

        assert_eq!(("Joao".to_string(), 3), game.read_winner());

        assert_eq!("game deleted: id -> 0", game.delete());
    }
}
