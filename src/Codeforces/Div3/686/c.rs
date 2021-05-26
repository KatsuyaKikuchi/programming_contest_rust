fn solve(v: &[usize]) -> usize {
    let n = v.len();
    let mut cnt = vec![0usize; n];
    for i in 0..n {
        if i == 0 || v[i] != v[i - 1] {
            cnt[v[i]] += 1;
        }
    }
    for i in 0..n {
        if cnt[i] > 0 {
            cnt[i] += 2;
        }
    }
    cnt[v[0]] -= 1;
    cnt[v[n - 1]] -= 1;

    cnt.into_iter()
        .filter(|&t| t > 0)
        .map(|t| t - 1).min().unwrap()
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let q = sc.read();
    for _ in 0..q {
        let n = sc.read();
        let mut v = vec![0; n];
        for i in 0..n {
            v[i] = sc.usize0();
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