fn get_index(v: &[i64], val: i64) -> usize {
    let (mut ret, mut ng) = (1, v.len());
    while (ng - ret) > 1 {
        let mid = (ng + ret) / 2;
        if v[mid] <= val {
            ret = mid;
        } else {
            ng = mid;
        }
    }
    ret
}

fn solve(v: &[(i64, i64)]) -> Vec<i64> {
    let n = v.len();
    let mut ret = vec![-1; n];
    let mut vals = vec![0];
    for i in 0..n {
        vals.push(v[i].0);
        vals.push(v[i].1);
    }
    vals.sort();
    vals.dedup();
    let mut mx = vec![(1_000_000_000_000i64, n); vals.len()];
    for i in 0..n {
        let (h, w) = (v[i].0, v[i].1);
        let hi = get_index(&vals, h);
        if mx[hi].0 > w {
            mx[hi] = (w, i);
        }
        let wi = get_index(&vals, w);
        if mx[wi].0 > h {
            mx[wi] = (h, i);
        }
    }
    for i in 1..mx.len() {
        if mx[i].0 > mx[i - 1].0 {
            mx[i] = mx[i - 1];
        }
    }
    for i in 0..n {
        let idx = get_index(&vals, v[i].0);
        if mx[idx - 1].0 < v[i].1 {
            ret[i] = mx[idx - 1].1 as i64 + 1;
        }
    }
    ret
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let q = sc.read::<usize>();
    for _ in 0..q {
        let n = sc.read();
        let mut v = Vec::new();
        for _ in 0..n {
            let t: (i64, i64) = (sc.read(), sc.read());
            v.push(t);
        }
        let ans = solve(&v);
        for i in 0..n {
            print!("{} ", ans[i]);
        }
        println!("");
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