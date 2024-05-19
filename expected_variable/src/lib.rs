pub use case::CaseExt;
pub use edit_distance::edit_distance;

pub fn expected_variable(input: &str, expected: &str) -> Option<String> {
    if input == "" || input.contains("-") || input.contains(" "){
        return None
    }
    let input1= format!("{}{}",input.split_at(1).0.to_lowercase(),input.split_at(1).1);
    let is_camel_case = input1.is_camel_lowercase();
    if !is_camel_case  && ! input.contains("_"){
        return None;
    }
    let distance = edit_distance(input.to_lowercase().as_str(), expected.to_lowercase().as_str());
    let max_length = expected.chars().count().max(input.chars().count()) as f64;
    let similarity = 1.0 - (distance as f64 / max_length);
    if similarity >= 0.5 {
        Some(format!("{}%", (similarity * 100.0).round()))
    } else {
        None
    }
}