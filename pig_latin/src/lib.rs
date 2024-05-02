pub fn pig_latin(text: &str) -> String {
    pig(text, true)
}

fn pig(text: &str, is_first: bool) -> String {
    if text.starts_with("qu") && !is_first {
        return pig(&format!("{}qu", &text[2..]), false);
    } else if "aeiou".contains(text.chars().next().unwrap().to_ascii_lowercase()) {
        return format!("{}ay", text);
    } else {
        return pig(&format!("{}{}", &text[1..], &text[0..1]), false);
    }
}
