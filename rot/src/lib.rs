pub fn rotate(text: &str, shift: i8) -> String {
    text.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let index = (c as u8 - base) as usize;
            let shifted_char = (((index as i8 + shift + 26) % 26) as u8 + base) as char;
            if c.is_ascii_uppercase() { shifted_char.to_ascii_uppercase() } else { shifted_char }
        } else {
            c
        }
    }).collect()
}
