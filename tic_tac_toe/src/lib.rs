pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let player_x = "X";
    let player_o = "O";

    if diagonals(player_x, &table) || horizontal(player_x, &table) || vertical(player_x, &table) {
        return "player X won".to_string();
    }
    if diagonals(player_o, &table) || horizontal(player_o, &table) || vertical(player_o, &table) {
        return "player O won".to_string();
    }

    "tie".to_string()
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    (table[0][0] == table[1][1] && table[1][1] == table[2][2] && table[2][2] == player) || (table[0][2] == table[1][1] && table[1][1] == table[2][0] && table[0][2] == player)
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for i in 0..3 {
        if table[i][0] == table[i][1] && table[i][1] == table[i][2] && table[i][2] == player {
            return true;
        }
    }
    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for i in 0..3 {
        if table[0][i] == table[1][i] && table[1][i] == table[2][i] && table[2][i] == player {
            return true;
        }
    }
    false
}
