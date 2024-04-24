
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
                for _char in 0..=count {
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
    for operation in v {
        let mut result = 0;
        let mut num_str = String::new();
        let mut last_op = '+';
        for c in operation.chars() {
            if c.is_digit(10) {
                num_str.push(c);
            } else {
                let num = num_str.parse::<i32>().unwrap();
                match last_op {
                    '+' => result += num,
                    '-' => result -= num,
                     => {}
                }
                numstr.clear();
                last_op = c;
            }
        }
        if !num_str.is_empty() {
            let num = num_str.parse::<i32>().unwrap();
            match last_op {
                '+' => result += num,
                '-' => result -= num,
                 => {}
            }
        }
        *operation = result.to_string();
    }
}

