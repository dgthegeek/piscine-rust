pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut result = String::new();
    for word in text.split_whitespace() {
        let mut chars: Vec<char> = word.chars().collect();
        match chars.get(0) {
            Some(&c) if vowels.contains(&c) => {
                result.push_str(&format!("{}ay ", word));
            }
            Some(&'s') if chars.get(1) == Some(&'q')&&chars.get(2) == Some(&'u') => {
                chars.remove(0);
                chars.remove(0);
                chars.remove(0);
                chars.push('s');
                chars.push('q');
                chars.push('u');
                let new_word: String = chars.into_iter().collect();
                result.push_str(&format!("{}ay", new_word));
            }
            Some(_) => {
                let mut consonants = String::new();
                while let Some(&c) = chars.get(0) {
                    if vowels.contains(&c) {
                        break;
                    }
                    consonants.push(c);
                    chars.remove(0);
                }
                chars.extend(consonants.chars());
                chars.extend(['a', 'y'].iter());
                let new_word: String = chars.into_iter().collect();
                result.push_str(&format!("{} ", new_word));
            }
            None => (),
        }
    }
    result.trim().to_string()
}