pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut i = 2;

    while primes.len() <= n as usize {
        if is_prime(i) {
            primes.push(i);
        }
        i += 1;
    }
    return primes[n as usize];

    fn is_prime(n: u32) -> bool {
        if n == 2 {
            return true;
        }

        if n % 2 == 0 {
            return false;
        }

        let mut i = 3;
        while i < n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        return true;
    }
}
