fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let s = sc.chars();
    let n = s.len();
    let md = 1_000_000_007i64;
    let mut dp = vec![vec![0; 13]; n + 1];
    dp[0][0] = 1i64;
    for i in 0..n {
        for j in 0..13usize {
            if s[i] != '?' {
                let m = s[i].to_digit(10).unwrap() as usize;
                dp[i + 1][(j * 10 + m) % 13] += dp[i][j];
            } else {
                for m in 0..10usize {
                    dp[i + 1][(j * 10 + m) % 13] += dp[i][j];
                }
            }
        }
        for j in 0..13usize {
            dp[i + 1][j] %= md;
        }
    }
    println!("{}", dp[n][5]);
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