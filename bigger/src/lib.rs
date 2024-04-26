use std::collections::HashMap;


pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut res = 0;
    for (_, num) in h{
        if num>res{
           res = num; 
        }
    }
    res
}

