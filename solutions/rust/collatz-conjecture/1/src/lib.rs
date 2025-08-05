pub fn collatz(n: u64) -> Option<u64> {
    let mut count = 1;
    
    let count_option = if n == 0 {
        return None;
    } else if n == 1 {
        Some(0)
    } else if n % 2 == 0 {
        count += collatz(n / 2).unwrap_or(0);

        Some(count)
    } else {
        count += collatz(3 * n + 1).unwrap_or(0);

        Some(count)
    };

    count_option
}
