pub fn num_to_ordinal(x: u32) -> String {
    let last_digit = x % 10;
    let last_two_digits = x % 100;

    match last_digit {
        1 => match last_two_digits {
            11 => format!("{}th", x),
            _ => format!("{}st", x),
        },
        2 => match last_two_digits {
            12 => format!("{}th", x),
            _ => format!("{}nd", x),
        },
        3 => match last_two_digits {
            13 => format!("{}th", x),
            _ => format!("{}rd", x),
        },
        _ => format!("{}th", x),
    }
}
