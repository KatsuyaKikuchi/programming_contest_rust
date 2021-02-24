use std::cmp::min;

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n = sc.read::<usize>();
    let mut v0 = vec![0; n];
    let mut v1 = vec![0; n];
    for i in 0..n {
        v0[i] = sc.read::<i64>();
    }
    for i in 0..n {
        v1[i] = sc.read::<i64>();
    }

    let md = 1_000_000_007i64;
    let mut ans = 1;
    let mut height = vec![-1; n];
    height[0] = v0[0];
    for i in 1..n {
        if v0[i] == v0[i - 1] {
            continue;
        }
        if v0[i] > v1[i] {
            ans = 0;
        } else {
            height[i] = v0[i];
        }
    }
    for i in (0..n).rev() {
        if i < n - 1 && v1[i + 1] == v1[i] {
            continue;
        }
        if (height[i] >= 0 && height[i] != v1[i]) || (v0[i] < v1[i]) {
            ans = 0;
        } else {
            height[i] = v1[i];
        }
    }
    for i in 0..n {
        if height[i] >= 0 {
            continue;
        }
        ans = (ans * min(v0[i], v1[i])) % md ;
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