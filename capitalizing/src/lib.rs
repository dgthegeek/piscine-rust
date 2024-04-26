pub fn capitalize_first(input: &str) -> String {
    let mut result = String::new();
    if let Some(first_char) = input.chars().nth(0){
        result.push(first_char.to_ascii_uppercase())
    }
    result+= &input[1..];
    result
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();

    for word in input.split_whitespace(){
        result+= &capitalize_first(word);
        if let Some(last_word) = input.split_whitespace().last(){
            if word != last_word{
                result.push(' ');
            }
        }
    }
    result
    
}

pub fn change_case(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars(){
        if c.is_ascii_lowercase(){
           let up=  c.to_uppercase().to_string();  
            result+= &up;
        } else if c.is_ascii_uppercase() {
            let low=  c.to_lowercase().to_string();  
            result+= &low;
        }else{
            result.push(' ');
        }
    };
    result
}

fn main() {
    println!("{}", capitalize_first("joe is missing"));
    println!("{}", title_case("jill is leaving A"));
    println!("{}",change_case("heLLo THere"));
}
