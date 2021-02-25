fn gcd(a: i64, b: i64) -> i64 {
    if b > a {
        return gcd(b, a);
    }
    let c = a % b;
    if c == 0 {
        b
    } else {
        gcd(b, c)
    }
}

fn solve(a: i64, b: i64, c: i64, d: i64) -> bool {
    if a < b || d < b {
        return false;
    }
    if c >= b {
        return true;
    }
    let (a, d) = (a % b, d % b);
    if d == 0 {
        if a <= c {
            return true;
        } else {
            return false;
        }
    }
    let g = gcd(b, d);
    let n = (a % g) - g + b;
    n <= c
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let q = sc.read::<usize>();
    for _i in 0..q {
        let (a, b, c, d) = (sc.read::<i64>(), sc.read::<i64>(),
                            sc.read::<i64>(), sc.read::<i64>());
        let ans = if solve(a, b, c, d) {
            "Yes"
        } else {
            "No"
        };
        println!("{}", ans);
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