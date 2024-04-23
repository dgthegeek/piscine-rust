
pub fn first_subword(mut s: String) -> String {
    let mut subword = String::new();

    let mut first_subword_started = false;

    for c in s.chars() {
        if c.is_uppercase() {
            if first_subword_started {
                break;
            }
            first_subword_started = true;
        } else if c == '_' || c.is_whitespace() {
            s = s.replacen(c, "", 1);
            break;
        }
        subword.push(c);
    }

    subword
}
