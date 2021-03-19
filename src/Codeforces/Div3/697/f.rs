fn check(a: &mut Vec<Vec<u32>>, b: &Vec<Vec<u32>>) -> bool {
    let n = a.len();
    for j in 0..n {
        if a[0][j] == b[0][j] {
            continue;
        }
        for i in 0..n {
            a[i][j] ^= 1;
        }
    }
    for i in 0..n {
        if a[i][0] == b[i][0] {
            continue;
        }
        for j in 0..n {
            a[i][j] ^= 1;
        }
    }

    for i in 0..n {
        for j in 0..n {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    true
}

fn solve(a: &Vec<Vec<u32>>, b: &Vec<Vec<u32>>) -> bool {
    let n = a.len();
    let mut x = a.clone();
    if check(&mut x, &b) {
        return true;
    }
    let mut x = a.clone();
    for i in 0..n {
        x[0][i] ^= 1;
    }
    if check(&mut x, &b) {
        return true;
    }
    false
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let q = sc.read::<usize>();
    for _ in 0..q {
        let n = sc.read();
        let mut a = vec![Vec::new(); n];
        for i in 0..n {
            a[i] = sc.chars().into_iter().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
        }
        let mut b = vec![Vec::new(); n];
        for i in 0..n {
            b[i] = sc.chars().into_iter().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
        }

        println!("{}", if solve(&a, &b) { "YES" } else { "NO" });
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