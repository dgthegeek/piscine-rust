pub fn get_diamond(letter: char) -> Vec<String> {
    if !('A'..='Z').contains(&letter) {
        return vec![String::from("Input must be a capital letter from A to Z.")];
    }

    let size = letter as u8 - b'A' + 1;
    let mut diamond = Vec::new();

    (0..size).chain((0..size - 1).rev()).for_each(|i| {
        let mut row = " ".repeat((size - i - 1) as usize);
        row.push((b'A' + i) as char);
        if i > 0 {
            row.push_str(&" ".repeat((2 * i - 1) as usize));
            row.push((b'A' + i) as char);
        }
        row.push_str(&" ".repeat((size - i - 1) as usize));
        diamond.push(row);
    });

    diamond
}
