use std::collections::VecDeque;

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let (n, m) = (sc.read::<usize>(), sc.read::<usize>());
    let mut to = vec![vec![]; n];
    let mut from = vec![vec![]; n];
    for _i in 0..m {
        let (a, b) = (sc.usize0(), sc.usize0());
        to[a].push(b);
        from[b].push(a);
    }

    let inf = 1_000_000_000i64;
    let mut ans = vec![];
    for i in 0..n {
        let mut min = (inf, 0);
        let mut q = VecDeque::new();
        q.push_back(i);
        let mut dist = vec![inf; n];
        dist[i] = 0;
        'main: while let Some(idx) = q.pop_front() {
            let cost = dist[idx] + 1;
            if cost >= min.0 {
                break;
            }
            for &nxt in to[idx].iter() {
                if i == nxt {
                    min = (cost, idx);
                    break 'main;
                }
                if dist[nxt] <= cost {
                    continue;
                }
                dist[nxt] = cost;
                q.push_back(nxt);
            }
        }
        if min.0 == inf {
            continue;
        }
        if ans.len() > 0 && ans.len() <= min.0 as usize {
            continue;
        }
        ans.clear();

        let mut q = VecDeque::new();
        q.push_back(min.1);
        while let Some(idx) = q.pop_front() {
            ans.push(idx);
            let cost = dist[idx] - 1;
            for &nxt in from[idx].iter() {
                if dist[nxt] != cost {
                    continue;
                }
                q.push_back(nxt);
                break;
            }
        }
    }

    if ans.len() == 0 {
        println!("-1");
    } else {
        println!("{}", ans.len());
        for v in ans {
            println!("{}", v + 1);
        }
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