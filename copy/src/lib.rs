pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let numbers = a.split_whitespace();
    let mut result = String::new();

    for number in numbers {
        if let Ok(num) = number.parse::<f64>() {
            let exp = num.exp().to_string();
            result.push_str(&exp);
            result.push(' ');
        }
    }

    if result.ends_with(' ') {
        result.pop();
    }

    (a, result)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut result = Vec::new();

    for number in &b {
        result.push((number.abs() as f64).ln());
    }

    (b, result)
}
