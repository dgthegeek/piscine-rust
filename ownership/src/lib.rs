

pub fn first_subword(s: String) -> String {
    let mut subword = String::new();
    
    let mut char_index = 0;
    for c in s.chars() {
        if c.is_uppercase() {
            if char_index == 0 {
                subword.push(c);
            } else {
                break;
            }
        } else if c == '_' || c == ' ' {
            break;
        } else {
            subword.push(c);
        }
        char_index += 1;
    }
    
    subword
}


