pub fn first_subword(s: String) -> String {
    let mut characters = s.chars().peekable();
    let mut subword = String::new();
    let mut is_first_character = true;

    while let Some(&next_char) = characters.peek() {
        if next_char.is_uppercase() && !is_first_character || next_char == '_' {
            break;
        }
        subword.push(next_char);
        characters.next();
        is_first_character = false;
    }

    subword.trim_end_matches('_').to_string()
}
