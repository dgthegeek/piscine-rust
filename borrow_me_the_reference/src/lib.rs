
pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let mut characters = s.chars().peekable();
    while let Some(&character) = characters.peek() {
        match character {
            '-' => {
                result.pop();
                characters.next();
            }
            '+' => {
                characters.next();
                let mut count = 0;
                while let Some(&next_character) = characters.peek() {
                    if next_character == '+' {
                        count += 1;
                        characters.next();
                    } else {
                        break;
                    }
                }
                for character in 0..=count {
                    characters.next();
                }
            }
            _ => {
                result.push(character);
                characters.next();
            }
        }
    }
    *s = result.into_iter().collect();
}

pub fn do_operations(v: &mut Vec<String>) {
    for equation in v.iter_mut() {
        let result: i32 = equation.split(|c| c == '+' || c == '-').map(|part| part.parse::<i32>().unwrap()).sum();
        *equation = result.to_string();
    }
}

