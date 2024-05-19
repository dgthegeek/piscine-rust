pub fn is_pangram(s: &str) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let lowercased_sentence = s.to_lowercase();
    alphabet.chars().all(|c| lowercased_sentence.contains(c))
}
