use std::cmp::max;

struct RollingHash {
    base: u128,
    modulo: u128,
}

impl RollingHash {
    fn new() -> Self {
        RollingHash {
            base: 9021,
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

pub struct UnionFind {
    n: usize,
    parent_or_size: Vec<i32>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            n: size,
            parent_or_size: vec![-1; size],
        }
    }

    pub fn find(&mut self, idx: usize) -> usize {
        assert!(idx < self.n);
        if self.parent_or_size[idx] < 0 {
            return idx;
        }
        self.parent_or_size[idx] = self.find(self.parent_or_size[idx] as usize) as i32;
        self.parent_or_size[idx] as usize
    }

    pub fn same(&mut self, idx0: usize, idx1: usize) -> bool {
        self.find(idx0) == self.find(idx1)
    }

    pub fn unit(&mut self, idx0: usize, idx1: usize) -> usize {
        let (idx0, idx1) = (self.find(idx0), self.find(idx1));
        if idx0 == idx1 {
            return idx0;
        }
        if -self.parent_or_size[idx0] < -self.parent_or_size[idx1] {
            return self.unit(idx1, idx0);
        }
        self.parent_or_size[idx0] += self.parent_or_size[idx1];
        self.parent_or_size[idx1] = idx0 as i32;
        idx0
    }

    pub fn size(&mut self, idx: usize) -> usize {
        let idx = self.find(idx);
        -self.parent_or_size[idx] as usize
    }


    pub fn group(&mut self) -> Vec<Vec<usize>> {
        let mut parent_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            parent_buf[i] = self.find(i);
            group_size[parent_buf[i]] += 1;
        }
        let mut result = vec![Vec::new(); self.n];
        for i in 0..self.n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            result[parent_buf[i]].push(i);
        }

        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let s = sc.chars();
    let t = sc.chars();
    let mut tmp = s.clone();
    while tmp.len() < t.len() {
        tmp.extend(&s);
    }
    let len = tmp.len();
    let target = tmp
        .clone()
        .into_iter()
        .chain(tmp)
        .map(|c| c as u64)
        .collect::<Vec<_>>();
    let sub = t.into_iter().map(|c| c as u64).collect::<Vec<_>>();

    let rolling_hash = RollingHash::new();
    let indices = rolling_hash.find(&target, &sub);
    let mut uf = UnionFind::new(len);
    for i in indices.into_iter().filter(|&x| x < len) {
        let nxt = (i + sub.len()) % len;
        if uf.same(i, nxt) {
            println!("-1");
            return;
        }
        uf.unit(i, nxt);
    }
    let ans = uf.group().into_iter().map(|v| v.len() - 1).max().unwrap();
    println!("{}", ans);
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        self.1.write_all(s.to_string().as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn usize0(&mut self) -> usize {
        self.read::<usize>() - 1
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}