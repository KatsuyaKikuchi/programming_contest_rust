use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n = sc.read::<usize>();
    let m = sc.read::<usize>();

    let mut v = vec![Vec::new(); n];
    let mut count = vec![0; n];
    for _i in 0..m {
        let (a, b, c) = (sc.usize0(), sc.usize0(), sc.read::<i64>());
        v[a].push((b, c));
        count[b] += 1;
    }

    let mut dist = vec![-1; n];
    let mut q = BinaryHeap::new();
    for i in 0..n {
        if count[i] == 0 {
            dist[i] = 0;
            q.push(Reverse((i, 0)));
        }
    }
    'main: while let Some(Reverse((idx, d))) = q.pop() {
        for &(nxt, c) in &v[idx] {
            let cost = c + d;
            if dist[nxt] == cost {
                continue;
            }

            if dist[nxt] >= 0 || cost > 1_000_000_000 {
                dist[nxt] = -1;
                break 'main;
            }

            dist[nxt] = cost;
            q.push(Reverse((nxt, cost)));
        }
    }

    println!("{}", if dist.contains(&-1) { "No" } else { "Yes" });
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