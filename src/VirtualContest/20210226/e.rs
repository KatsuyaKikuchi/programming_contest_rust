use std::collections::HashMap;

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n = sc.read::<usize>();
    let mut v = vec![];
    for _ in 0..n {
        let a = sc.read::<i32>();
        v.push(a);
    }
    let mut dp = vec![0; n + 1];
    dp[0] = 1i64;
    let mut hash = HashMap::new();
    let md = 1_000_000_007i64;
    for i in 0..n {
        dp[i + 1] += dp[i];
        if let Some(&p) = hash.get(&v[i]) {
            if p + 1 < i {
                dp[i + 1] += dp[p + 1];
            }
        }
        let t = hash.entry(v[i]).or_insert(0);
        *t = i;
        dp[i + 1] %= md;
    }
    println!("{}", dp[n]);
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