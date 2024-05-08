pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut numbers: Vec<u32> = Vec::new();

    for token in s.split_whitespace() {
        if token.ends_with('k') {
            
            let mut value = token[0..token.len()-1].parse::<f64>().unwrap();
            value *= 1000.0;
            
            numbers.push(value as u32);
        }else{
            let value = token.parse::<u32>().unwrap_or(0);
            numbers.push(value);
        }


    }
    Box::new(numbers)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
