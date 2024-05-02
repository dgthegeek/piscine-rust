pub fn stars(n: u32) -> String {
    let stars_count = 2u32.pow(n);
    (0..stars_count)
        .map(|_| '*')
        .collect::<String>()
}
