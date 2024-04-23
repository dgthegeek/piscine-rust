pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();

    for name in names {
        let mut initials = String::new();
        let mut first_word = true;

        for word in name.split_whitespace() {
            if !first_word {
                initials.push(' ');
            } else {
                first_word = false;
            }

            if let Some(first_char) = word.chars().nth(0) {
                initials.push(first_char);
                if !first_word{
                    initials.push('.');
                }
            }
        }
        result.push(initials);
    }
    result
}
