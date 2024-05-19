pub fn first_fifty_even_square() -> Vec<i32> {
    let even_numbers = (1..)
        .filter(|&x| x % 2 == 0)
        .take(50);
    
    let squared_numbers: Vec<i32> = even_numbers.map(|x| x * x).collect();
    squared_numbers
}
