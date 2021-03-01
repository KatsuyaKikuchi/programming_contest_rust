use std::cmp::min;

fn solve() -> bool {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let mut v0 = vec![];
    let mut v1 = vec![];
    let mut d = 0;
    for i in 0..n {
        let s = sc.chars();
        let mut t = (0, 0);
        for c in s {
            if c == '(' {
                t.0 += 1;
            } else {
                t.0 -= 1;
                t.1 = min(t.0, t.1);
            }
        }
        if t.0 >= 0 && t.1 >= 0 {
            d += t.0;
        } else if t.0 >= 0 {
            v0.push(t);
        } else {
            v1.push(t);
        }
    }
    v0.sort_by(|a, b| b.1.cmp(&a.1));
    for t in v0 {
        if d + t.1 < 0 {
            return false;
        }
        d += t.0;
    }
    v1.sort_by(|a, b| (b.0 - b.1).cmp(&(a.0 - a.1)));
    for t in v1 {
        if d + t.1 < 0 {
            return false;
        }
        d += t.0;
    }

    d == 0
}

fn main()
{
    println!("{}", if solve() { "Yes" } else { "No" });
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