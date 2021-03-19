use std::collections::VecDeque;

fn dfs(mut graph: &mut Vec<Vec<usize>>, v: &[(usize, usize)]) -> usize {
    if v.len() == 1 {
        return v[0].1;
    };
    let n = v.len();
    let mut mi = 0;
    for i in 0..n {
        if v[i].0 > v[mi].0 {
            mi = i;
        }
    }
    let root = v[mi].1;
    let left = &v[0..mi];
    let right = &v[(mi + 1)..n];
    if left.len() > 0 {
        let child = dfs(&mut graph, &left);
        graph[root].push(child);
    }
    if right.len() > 0 {
        let child = dfs(&mut graph, &right);
        graph[root].push(child);
    }
    root
}

fn solve(v: &[(usize, usize)]) -> Vec<i32> {
    let n = v.len();
    let mut dist = vec![0; n];
    let mut graph = vec![vec![]; n];
    let root = dfs(&mut graph, &v);
    let mut q = VecDeque::new();
    q.push_back((root, 0));
    while let Some((idx, dst)) = q.pop_front() {
        dist[idx] = dst;
        for &nxt in graph[idx].iter() {
            q.push_back((nxt, dst + 1));
        }
    }
    dist
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let q = sc.read::<usize>();
    for _ in 0..q {
        let n = sc.read::<usize>();
        let mut v = vec![(0, 0); n];
        for i in 0..n {
            v[i].0 = sc.read::<usize>() - 1;
            v[i].1 = i;
        }

        let ans = solve(&v);
        for t in ans {
            print!("{} ", t);
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