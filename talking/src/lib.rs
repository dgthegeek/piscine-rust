pub fn talking(text: &str) -> &str {
    match text.trim() {
        "" => "Just say something!",
        t if t == t.to_uppercase() && t.chars().any(|c| c.is_alphabetic()) => {
            if t.ends_with('?') {
                "Quiet, I am thinking!"
            } else {
                "There is no need to yell, calm down!"
            }
        }
        t if t.ends_with('?') => "Sure.",
        _ => "Interesting",
    }
}
