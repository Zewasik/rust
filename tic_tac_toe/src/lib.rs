pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let players = vec!["X", "O"];
    let mut win = "Tie".to_string();
    for player in players {
        if diagonals(player, &table) || horizontal(player, &table) || vertical(player, &table) {
            win = ["player", player, "won"].join(" ");
            break;
        }
    }

    return win.to_string();
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    return player == table[1][1]
        && ((player == table[0][0] && player == table[2][2])
            || (player == table[2][0] && player == table[0][2]));
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    return (player == table[0][0] && player == table[1][0] && player == table[2][0])
        || (player == table[0][1] && player == table[1][1] && player == table[2][1])
        || (player == table[0][2] && player == table[1][2] && player == table[2][2]);
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    return (player == table[0][0] && player == table[0][1] && player == table[0][2])
        || (player == table[1][0] && player == table[1][1] && player == table[1][2])
        || (player == table[2][0] && player == table[2][1] && player == table[2][2]);
}
