

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = (c as f64).exp();
    let abs_log = (c.abs() as f64).ln();
    (c, exp, abs_log)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values: Vec<String> = a
        .split_whitespace()
        .map(|x| {
            let num: f64 = x.parse().unwrap_or(0.0);
            num.exp().to_string()
        })
        .collect();

    (a, exp_values.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let abs_logs: Vec<f64> = b.iter().map(|&x| (x.abs() as f64).ln()).collect();
    (b, abs_logs)
}


