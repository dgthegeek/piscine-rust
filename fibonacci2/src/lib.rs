pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut fib = (0, 1);
    for _ in 2..=n {
        fib = (fib.1, fib.0 + fib.1);
    }
    fib.1
}
