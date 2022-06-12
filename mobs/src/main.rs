use mobs::*;

fn main() {
    let (mafia1, mafia2) = (
        Mob {
            name: "Hairy Giants".to_string(),
            boss: Boss::new("Louie HaHa".to_string(), 36),
            cities: vec![("San Francisco".to_string(), 7)],
            members: vec![
                Member::new("Benny Eggs".to_string(), member::Role::Soldier, 28),
                Member::new("Jhonny".to_string(), member::Role::Associate, 17),
                Member::new("Greasy Thumb".to_string(), member::Role::Soldier, 30),
                Member::new("No Finger".to_string(), member::Role::Caporegime, 32),
            ],
            wealth: 100000,
        },
        Mob {
            name: "Red Thorns".to_string(),
            boss: Boss::new("Big Tuna".to_string(), 30),
            cities: vec![("San Jose".to_string(), 5)],
            members: vec![
                Member::new("Knuckles".to_string(), member::Role::Soldier, 25),
                Member::new("Baldy Dom".to_string(), member::Role::Caporegime, 36),
                Member::new("Crazy Joe".to_string(), member::Role::Underboss, 23),
            ],
            wealth: 70000,
        },
    );

    println!("{:?}\n{:?}", mafia1, mafia2);
}
