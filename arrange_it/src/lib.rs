pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    // Sort words based on the position number
    words.sort_by_key(|word| {
        let position: usize = word.chars().find(|c| c.is_digit(10)).unwrap().to_digit(10).unwrap() as usize;
        position
    });

    let mut result = String::new();
    for word in words {
        let word_without_num: String = word.chars().filter(|c| !c.is_digit(10)).collect();
        result.push_str(&word_without_num);
        result.push(' ');
    }

    result.trim().to_string()
}