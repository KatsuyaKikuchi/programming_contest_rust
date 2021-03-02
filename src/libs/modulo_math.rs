pub fn mod_pow(n: i64, r: i64, m: i64) -> i64 {
    let mut ret = 1;
    let mut p = n;
    let mut r = r.clone();
    while r > 0 {
        if (r & 1) == 1 {
            ret = (ret * p) % m;
        }
        p = (p * p) % m;
        r >>= 1;
    }
    ret
}

pub struct ModCombination {
    size: usize,
    modulo: i64,

    factional: Vec<i64>,
    inv_factional: Vec<i64>,
}

impl ModCombination {
    pub fn new(size: usize, modulo: i64) -> Self {
        let mut factional = vec![1; size + 1];
        for i in 1..=size {
            factional[i] = factional[i - 1] * (i as i64) % modulo;
        }
        let mut inv_factional = vec![1; size + 1];
        inv_factional[size] = mod_pow(factional[size], modulo - 2, modulo);
        for i in (1..size).rev() {
            inv_factional[i] = inv_factional[i + 1] * ((i + 1) as i64) % modulo;
        }
        ModCombination {
            size: size,
            modulo: modulo,
            factional: factional,
            inv_factional: inv_factional,
        }
    }

    pub fn calc_c(&self, n: usize, r: usize) -> i64 {
        assert!(n <= self.size && r <= self.size,
                "args is out of size!! size = {}, n={}, r={}", self.size, n, r);
        if n < r {
            return 0;
        }
        ((self.factional[n] * self.inv_factional[r] % self.modulo)
            * self.inv_factional[n - r]) % self.modulo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_combination_test() {
        let size = 100000;
        let modulo = 1_000_000_007;
        let cmb = ModCombination::new(size, modulo);
        assert_eq!(cmb.calc_c(3, 11), 0);
        assert_eq!(cmb.calc_c(size - 1, size), 0);
        for n in 0..10 {
            for r in 0..=n {
                let (mut val, mut inv) = (1, 1);
                for i in 0..r {
                    val *= (n - i) as i64;
                    inv *= (r - i) as i64;
                }
                assert_eq!(cmb.calc_c(n, r), val / inv);
            }
        }
        for r in 0..=size {
            assert!(cmb.calc_c(size, r) < modulo);
        }
    }

    #[test]
    #[should_panic]
    fn mod_combination_out_of_size_n() {
        let size = 10;
        let cmb = ModCombination::new(size, 3);
        cmb.calc_c(size + 1, 0);
    }

    #[test]
    #[should_panic]
    fn mod_combination_out_of_size_r() {
        let size = 10;
        let cmb = ModCombination::new(size, 3);
        cmb.calc_c(0, size + 1);
    }
}