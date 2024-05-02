pub use edit_distance::edit_distance;
use std::cmp::max;

pub fn is_camel_case(s: &str) -> bool {
    let mut has_lower = false;
    for c in s.chars() {
        if c.is_lowercase() {
            has_lower = true;
        } else if c.is_uppercase() {
            if has_lower {
                return true;
            }
        }
    }
    false
}

pub fn is_snake_case(s: &str) -> bool {
    s.chars().any(|c| c == '_')
}

pub fn calculate_similarity(input: &str, expected: &str) -> f64 {
    let distance = edit_distance(input.to_lowercase().as_str(), expected.to_lowercase().as_str());
    let max_length = max(input.len(), expected.len());
    ((max_length - distance) as f64 / max_length as f64) * 100.0
}

pub fn expected_variable(input: &str, expected: &str) -> Option<String> {
    if is_camel_case(input) || is_snake_case(input) {
        let similarity = calculate_similarity(input, expected);
        if similarity > 50.0 {
            return Some(format!("{:.0}%", similarity));
        }
    }
    None
}
