use std::collections::{VecDeque};
use std::cmp::{min};

fn dfs(graph: &Vec<Vec<usize>>, idx: usize, dist: &[usize], mut min_dist: &mut [usize]) -> usize {
    if min_dist[idx] <= dist[idx] {
        return min_dist[idx];
    }
    let mut ret = dist[idx];
    for &nxt in graph[idx].iter() {
        if dist[nxt] <= dist[idx] {
            ret = min(ret, dist[nxt]);
        } else {
            let d = dfs(&graph, nxt, &dist, &mut min_dist);
            ret = min(d, ret);
        }
    }

    min_dist[idx] = ret;
    ret
}

fn solve(graph: Vec<Vec<usize>>) -> Vec<usize> {
    let n = graph.len();
    let mut ret = vec![n; n];
    let mut dist = vec![n; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    dist[0] = 0;
    while let Some(idx) = q.pop_front() {
        let d = dist[idx];
        for &nxt in graph[idx].iter() {
            if dist[nxt] <= d + 1 {
                continue;
            }
            dist[nxt] = d + 1;
            q.push_back(nxt);
        }
    }
    dfs(&graph, 0, &dist, &mut ret);

    ret
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let q = sc.read();
    for _ in 0..q {
        let (n, m) = (sc.read(), sc.read());
        let mut graph = vec![vec![]; n];
        for _ in 0..m {
            let (a, b) = (sc.usize0(), sc.usize0());
            graph[a].push(b);
        }

        let ans = solve(graph);
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