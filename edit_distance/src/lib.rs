pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut distances = vec![vec![0; len2 + 1]; len1 + 1];
    for i in 0..=len1 {
        distances[i][0] = i;
    }
    for j in 0..=len2 {
        distances[0][j] = j;
    }
    for (i, char1) in s1.chars().enumerate() {
        for (j, char2) in s2.chars().enumerate() {
            let cost = if char1 == char2 { 0 } else { 1 };
            distances[i + 1][j + 1] = *vec![
                distances[i][j + 1] + 1,
                distances[i + 1][j] + 1,
                distances[i][j] + cost,
            ]
            .iter()
            .min()
            .unwrap();
        }
    }
    distances[len1][len2]
}