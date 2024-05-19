pub fn factorial(num: u64) -> u64 {
    if num == 0 || num == 1 {
        return 1;
    }

    let mut result = 1;
    for c in 1..=num {
        result *= c;
    }

    result
}
