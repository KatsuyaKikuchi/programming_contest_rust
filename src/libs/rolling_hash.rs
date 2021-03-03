struct RollingHash {
    base: u128,
    modulo: u128,
}

impl RollingHash {
    fn new() -> Self {
        RollingHash {
            base: 9021, //< todo
            modulo: (1u128 << 61) - 1,
        }
    }

    fn convert(&self, v: u64) -> u128 {
        v as u128 + 1
    }

    fn calc_hash(&self, s: &[u64]) -> u128 {
        let mut hash = 0;
        let mut base = 1;
        for &val in s.iter().rev() {
            let val = self.convert(val);
            hash = (hash + val * base) % self.modulo;
            base = (base * self.base) % self.modulo;
        }
        hash
    }

    fn find(&self, target: &[u64], sub: &[u64]) -> Vec<usize> {
        let mut start_indices = Vec::<usize>::new();
        if sub.len() == 0 || target.len() < sub.len() {
            return start_indices;
        }
        let sub_hash = self.calc_hash(&sub);
        let mut target_hash = self.calc_hash(&target[0..sub.len()]);
        if sub_hash == target_hash {
            start_indices.push(0);
        }
        let p = (0..sub.len() - 1).fold(1, |v, _| (v * self.base) % self.modulo);
        for i in sub.len()..target.len() {
            let (pop, push) = (self.convert(target[i - sub.len()]), self.convert(target[i]));
            target_hash = (self.modulo + target_hash - pop * p % self.modulo) % self.modulo;
            target_hash = (target_hash * self.base % self.modulo + push) % self.modulo;
            if target_hash == sub_hash {
                start_indices.push(i - sub.len() + 1);
            }
        }
        start_indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rolling_hash_test() {
        let rolling_hash = RollingHash::new();
        let a = [1, 1, 1, 1];
        let b = [1, 1];
        assert_eq!(rolling_hash.find(&a, &b), [0, 1, 2]);
        let b = [1];
        assert_eq!(rolling_hash.find(&a, &b), [0, 1, 2, 3]);
        let a = [1, 2, 5, 6, 7, 1, 3, 4, 6, 5, 6];
        let b = [5, 6];
        assert_eq!(rolling_hash.find(&a, &b), [2, 9]);
        let a = [];
        assert_eq!(rolling_hash.find(&a, &b), []);
        let a = [1, 2, 3, 4, 4, 5, 6, 7, 7, 8];
        let b = [9];
        assert_eq!(rolling_hash.find(&a, &b), []);
    }
}