
pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let mut diag1_count = 0;
    let mut diag2_count = 0;
    let n = table.len();

    for i in 0..n {
        if table[i][i] == player {
            diag1_count += 1;
        }
        if table[i][n - i - 1] == player {
            diag2_count += 1;
        }
    }

    diag1_count == n || diag2_count == n
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for row in table {
        if row.iter().all(|&cell| cell == player) {
            return true;
        }
    }
    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let n = table.len();
    for col in 0..n {
        let mut count = 0;
        for row in 0..n {
            if table[row][col] == player {
                count += 1;
            }
        }
        if count == n {
            return true;
        }
    }
    false
}

pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    let players = ["X", "O"];

    for &player in &players {
        if diagonals(player, &table) || horizontal(player, &table) || vertical(player, &table) {
            return format!("player {} won", player);
        }
    }

    "tie".to_string()
}
