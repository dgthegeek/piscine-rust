pub fn capitalize_first(input: &str) -> String {
    let mut output = String::new();
    for (index, value) in input.chars().enumerate() {
        if index == 0 {
            output.push(value.to_uppercase().next().unwrap());
            continue;
        }
        output.push(value);
    }

    output
}

pub fn title_case(input: &str) -> String {
    let words: Vec<&str> = input.split(" ").collect();
    let mut output = String::new();
    for word in words {
        let word = capitalize_first(word);
        output.push_str(&word);
        output.push(' ');
    }

    output.trim_end().to_string()
}

pub fn change_case(input: &str) -> String {
    let mut output = String::new();
    for char in input.chars() {
        if char.is_uppercase() {
            output.push(char.to_lowercase().next().unwrap());
        } else if char.is_lowercase() {
            output.push(char.to_uppercase().next().unwrap());
        } else {
            output.push(char);
        }
    }

    output
}