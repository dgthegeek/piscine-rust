pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.chars().count();
    let n = target.chars().count();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Initialize the dp matrix with base cases
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    // Populate the dp matrix
    for (i, src_char) in source.chars().enumerate() {
        for (j, tgt_char) in target.chars().enumerate() {
            if src_char == tgt_char {
                dp[i + 1][j + 1] = dp[i][j];
            } else {
                dp[i + 1][j + 1] = dp[i][j].min(dp[i][j + 1].min(dp[i + 1][j])) + 1;
            }
        }
    }

    dp[m][n]
}


