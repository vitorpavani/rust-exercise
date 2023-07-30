pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    let mut count = 0;
    if n == 0 {
        return None;
    }
    loop {
        if n >= u64::MAX / 3 {
            return None;
        }
        if n == 1 {
            return Some(count);
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        count += 1;
    }
}