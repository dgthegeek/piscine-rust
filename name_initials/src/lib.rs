pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials = Vec::new();
    for name in names {
        let words = name.split_whitespace();
        let mut initial = String::new();
        for word in words {
            if let Some(first_char) = word.chars().next() {
                initial.push(first_char);
                initial.push_str(". ");
            }
        }  
        initial.pop();
        initials.push(initial);
    }

    initials
}