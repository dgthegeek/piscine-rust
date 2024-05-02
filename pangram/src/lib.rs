pub fn is_pangram(sentence: &str) -> bool {
    let mut chars: Vec<char> = sentence.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    chars.sort();
    chars.dedup();
    chars.len() >= 26
}
