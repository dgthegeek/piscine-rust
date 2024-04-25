pub fn capitalize_first(input: &str) -> String {
    if let Some(first_char) = input.chars().next() {
        let capitalized = first_char.to_uppercase();
        let mut result = capitalized.to_string();
        result.push_str(&input[1..]);
        result
    } else {
        input.to_string() // Return input unchanged if it's an empty string
    }
}

pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first_char) => {
                    let mut capitalized = first_char.to_uppercase();
                    capitalized.extend(chars);
                    capitalized
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c
            }
        })
        .collect()
}
