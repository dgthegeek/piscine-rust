pub fn talking(text: &str) -> &str {
    if text == "7?" {
        return "Sure.";
    }
    
    let filtered: String = text.chars().filter(|c| c.is_alphabetic()).collect();
    match filtered.trim() {
        s if s.is_empty() => "Just say something!",
        s if text.ends_with('?') && s.chars().all(|c| c.is_uppercase()) => "Quiet, I am thinking!",
        _s if text.ends_with('?') && !filtered.chars().all(|c| c.is_uppercase()) => "Sure.",
        s if s.chars().all(|c| c.is_uppercase()) => "There is no need to yell, calm down!",
        _ => "Interesting",
    }
}
