use std::cmp::min;

fn solve(x: i64) -> i64 {
    let v = (1..=9).collect::<Vec<i64>>();
    let bit = 1 << v.len();
    let inf = 1_000_000_000_000i64;
    let mut ans = inf;
    for b in 0..bit {
        let mut vs = vec![];
        for i in 0..v.len() {
            if ((b >> i) & 1) == 1 {
                vs.push(v[i]);
            }
            let sum = vs.iter().sum::<i64>();
            if sum != x {
                continue;
            }
            let val = vs.iter().fold(0, |s, t| s * 10 + t);
            ans = min(ans, val);
        }
    }
    if ans == inf {
        ans = -1;
    }
    ans
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let q = sc.read();
    for _ in 0..q {
        let x = sc.read();
        println!("{}", solve(x));
    }
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