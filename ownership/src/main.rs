fn main() {
	let s1 = String::from("helloWorld");
	let s2 = String::from("snake_case");
	let s3 = String::from("CamelCase");
	let s4 = String::from("just");

	println!("first_subword({}) = {}", s1.clone(), first_subword(s1));
	println!("first_subword({}) = {}", s2.clone(), first_subword(s2));
	println!("first_subword({}) = {}", s3.clone(), first_subword(s3));
	println!("first_subword({}) = {}", s4.clone(), first_subword(s4));
}

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

