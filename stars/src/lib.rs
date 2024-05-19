pub fn stars(n: u32) -> String {
    let mut stars = String::new();
    let pow = u32::pow(2, n);
    
    for _ in 0..pow {
        stars.push('*');
    }

    stars
}