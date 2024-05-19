pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut vec: Vec<usize> = Vec::new();
    if arr.len() == 1 {return vec;}
    for curr_ind in 0..arr.len() {
        let mut product = 1;
        for ind in 0..arr.len() {
            if ind != curr_ind {
                product *= arr[ind];
            }
        }
        vec.push(product);
    }
    vec
}