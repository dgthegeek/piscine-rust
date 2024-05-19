pub fn number_logic(num: u32) -> bool {
    let num_to_str = num.to_string(); 
    let num_digits = num_to_str.len() as u32;
    
    let sum_of_powers: u32 = num_to_str.chars() 
        .map(|c| c.to_digit(10).unwrap().pow(num_digits))
        .sum(); 
    num == sum_of_powers 
}
