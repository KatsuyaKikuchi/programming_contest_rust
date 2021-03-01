pub fn primes(n: usize) -> Vec<i32> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; n + 1];
    for p in 2..=n {
        if !is_prime[p] {
            continue;
        }
        primes.push(p as i32);
        let mut num = 2 * p;
        while num <= n {
            is_prime[num] = false;
            num += p;
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_test() {
        let p = primes(20);
        assert_eq!(p, [2, 3, 5, 7, 11, 13, 17, 19]);
        let p = primes(2);
        assert_eq!(p, [2]);
        let p = primes(100000);
        assert_eq!(p.len(), 9592);
        let p = primes(10000000);
        assert_eq!(p[9999], 104729);
    }
}