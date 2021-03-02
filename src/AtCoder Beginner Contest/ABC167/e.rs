fn mod_pow(n: i64, r: i64, m: i64) -> i64 {
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

struct ModCombination {
    size: usize,
    modulo: i64,

    factional: Vec<i64>,
    inv_factional: Vec<i64>,
}

impl ModCombination {
    fn new(size: usize, modulo: i64) -> Self {
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

    fn calc_c(&self, n: usize, r: usize) -> i64 {
        assert!(n <= self.size && r <= self.size);
        if n < r {
            return 0;
        }
        ((self.factional[n] * self.inv_factional[r] % self.modulo)
            * self.inv_factional[n - r]) % self.modulo
    }
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let (n, m, k): (i64, i64, usize) = (sc.read(), sc.read(), sc.read());
    let mut ans = 0;
    let modulo = 998244353;
    let cmb = ModCombination::new(n as usize, modulo);
    for i in 0..=k {
        let len = n - (i as i64) - 1;
        let p = m * mod_pow(m - 1, len, modulo) % modulo;
        ans += p * cmb.calc_c((n - 1) as usize, i) % modulo;
        ans %= modulo;
    }
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