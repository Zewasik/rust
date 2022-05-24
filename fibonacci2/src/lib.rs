pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n >= 2 {
        return fibonacci(n - 1) + fibonacci(n - 2);
    } else {
        return 1;
    }
}
