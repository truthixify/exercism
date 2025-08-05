pub fn factors(n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    let mut n = n;

    while n % 2 == 0 {
        res.push(2);

        n /= 2;
    }

    let mut i = 3;

    while i <= n.isqrt() {
        while n % i == 0  {
            res.push(i);
    
            n /= i;
        }

        i += 2;
    }

    if n > 2 {
        res.push(n);
    }

    res
}
