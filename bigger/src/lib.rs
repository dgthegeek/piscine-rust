use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut biggest = 0;
    for value in h.values() {
        if *value > biggest {
            biggest = *value;
        }
    }
    biggest
}