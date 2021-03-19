use std::cmp::min;

fn solve(a: &[i64], b: &[i64], m: i64) -> i64 {
    let mut low = vec![];
    let mut heigh = vec![];
    for i in 0..a.len() {
        if b[i] == 1 {
            low.push(a[i]);
        } else {
            heigh.push(a[i]);
        }
    }
    low.sort_by(|a, b| b.cmp(a));
    heigh.sort_by(|a, b| b.cmp(a));
    let inf = 1_000_000_000_000_000i64;
    let mut ans = inf;
    let mut s = vec![0];
    for i in 0..heigh.len() {
        let val = s.last().unwrap() + heigh[i];
        s.push(val);
    }
    low.push(inf);
    let mut sum = 0;
    for i in 0..low.len() {
        let (mut ok, mut ng) = (s.len() as i64, -1);
        while (ok - ng).abs() > 1 {
            let mid = ((ok + ng) / 2) as usize;
            if sum + s[mid] >= m {
                ok = mid as i64;
            } else {
                ng = mid as i64;
            }
        }
        if ok != s.len() as i64 {
            ans = min(ok * 2 + i as i64, ans);
        }
        sum += low[i];
    }

    if ans == inf {
        -1
    } else {
        ans
    }
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let q = sc.read::<usize>();
    for _ in 0..q {
        let (n, m) = (sc.read(), sc.read::<i64>());
        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = sc.read::<i64>();
        }
        let mut b = vec![0; n];
        for i in 0..n {
            b[i] = sc.read::<i64>();
        }

        println!("{}", solve(&a, &b, m));
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