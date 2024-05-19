pub fn mean(list: &Vec<i32>) -> f64 {
    let list_sum: i32 = list.iter().sum();
    let mean = list_sum as f64 / list.len() as f64;
    mean
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut sorted_list = list.clone();
    sorted_list.sort();

    let list_len = sorted_list.len();

    if list_len % 2 == 0 {
        let middle_right = list_len / 2;
        let middle_left = middle_right - 1;
        (sorted_list[middle_right] + sorted_list[middle_left]) / 2
    } else {
        sorted_list[list_len / 2]
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut mode = 0;
    let mut mode_count = 0;

    for number in list {
        let count = list.iter().filter(|&x| x == number).count();
        if count > mode_count {
            mode = *number;
            mode_count = count;
        }
    }

    mode
}