pub fn rotate(input: &str, key: i8) -> String {
    let nkey = if key < 0 {(26+key) as u8} else {key as u8};
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let rotated = (c as u8 - base + nkey as u8) % 26 + base;
            
            rotated as char
        } else {
            c
        }
    }).collect()
}