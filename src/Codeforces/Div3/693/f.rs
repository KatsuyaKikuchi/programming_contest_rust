fn solve(v: &[(i64, usize)]) -> bool {
    let mut pos = [0; 2];
    let mut vs = vec![vec![]; 2];
    for &(c, r) in v.iter() {
        vs[r].push(c);
    }
    vs[0].push(1_000_000_000_000i64);
    vs[1].push(1_000_000_000_000i64);
    let mut idx = vec![0; 2];
    while idx[0] < vs[0].len() - 1 || idx[1] < vs[1].len() - 1 {
        if pos[0] != pos[1] {
            let (c0, c1) = (vs[0][idx[0]], vs[1][idx[1]]);
            if vs[0][idx[0]] == vs[1][idx[1]] {
                if c0 % 2 != pos[0] || c1 % 2 != pos[1] {
                    return false;
                }
                idx[0] += 1;
                idx[1] += 1;
            } else {
                if vs[0][idx[0]] < vs[1][idx[1]] {
                    if c0 % 2 != pos[0] {
                        return false;
                    }
                    idx[0] += 1;
                } else {
                    if c1 % 2 != pos[1] {
                        return false;
                    }
                    idx[1] += 1;
                }
            }
            pos[0] = 0;
            pos[1] = 0
        } else {
            let (mut r, mut c) = (0, 0);
            if vs[0][idx[0]] < vs[1][idx[1]] {
                r = 0;
                c = vs[0][idx[0]];
                idx[0] += 1;
            } else {
                r = 1;
                c = vs[1][idx[1]];
                idx[1] += 1;
            }
            pos[r] = (c + 1) % 2;
            pos[(r + 1) % 2] = c % 2;
        }
    }
    pos[0] == pos[1]
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let q = sc.read();
    for _ in 0..q {
        let (_, m) = (sc.read::<i64>(), sc.read());
        let mut v = Vec::new();
        for _ in 0..m {
            let (r, c) = (sc.read::<usize>() - 1, sc.read::<i64>());
            v.push((c, r));
        }
        v.sort();
        println!("{}", if solve(&v) { "YES" } else { "NO" });
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