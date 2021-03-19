use std::cmp::{min, max};

fn solve(v: &[i32]) -> i32 {
    let n = v.len();
    let mut ans = 0;
    for i in 1..n {
        let (mut mn, mx) = (min(v[i - 1], v[i]), max(v[i - 1], v[i]));
        while mn * 2 < mx {
            mn *= 2;
            ans += 1;
        }
    }
    ans
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let q = sc.read::<usize>();
    for _ in 0..q {
        let n = sc.read::<usize>();
        let mut v = vec![0; n];
        for i in 0..n {
            v[i] = sc.read();
        }
        println!("{}", solve(&v));
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