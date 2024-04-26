pub fn mean(list: &Vec<i32>) -> f64 {
    let sum: i32 = list.iter().sum();
    let count = list.len() as f64;
    sum as f64 / count
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut sorted_list = list.clone();
    sorted_list.sort();
    let len = sorted_list.len();
    if len % 2 == 0 {
        (sorted_list[len / 2 - 1] + sorted_list[len / 2]) / 2
    } else {
        sorted_list[len / 2]
    }
}

use std::collections::HashMap;

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for &value in list {
        *counts.entry(value).or_insert(0) += 1;
    }
    let (&mode, _) = counts.iter().max_by_key(|&(_, &count)| count).unwrap();
    mode
}
