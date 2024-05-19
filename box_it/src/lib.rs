pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut numbers_without_suffix = Vec::new();

    for number in s.split_whitespace() {
        if number.ends_with("k") {
            let mut str_number = String::from(number);
            str_number.pop();
            numbers_without_suffix.push((str_number.parse::<f64>().unwrap() * 1000.0) as u32);
        } else {
            numbers_without_suffix.push(number.to_string().parse::<u32>().unwrap());
        }
    }

    Box::new(numbers_without_suffix)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}