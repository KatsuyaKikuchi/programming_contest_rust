use std::cmp::min;

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n = sc.read::<usize>();
    let mut pos = vec![];
    for _i in 0..n {
        let p = (sc.read::<i64>(), sc.read::<i64>());
        pos.push(p);
    }
    pos.sort();
    let mut ans = n;
    for i in 0..n {
        for j in (i + 1)..n {
            let (p, q) = (pos[j].0 - pos[i].0, pos[j].1 - pos[i].1);
            let mut used = vec![false; n];

            let mut sum = 0;
            for i in 0..n {
                if used[i] {
                    continue;
                }
                sum += 1;
                used[i] = true;
                let (mut x, mut y) = (pos[i].0 + p, pos[i].1 + q);
                for j in (i + 1)..n {
                    if used[j] {
                        continue;
                    }
                    if pos[j].0 != x || pos[j].1 != y {
                        continue;
                    }
                    used[j] = true;
                    x += p;
                    y += q;
                }
            }
            ans = min(ans, sum);
        }
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