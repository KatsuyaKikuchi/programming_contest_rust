fn dfs(graph: &Vec<Vec<usize>>, idx: usize) -> (i32, i32) {
    let mut even = vec![];
    let mut odd = vec![];
    for &nxt in graph[idx].iter() {
        let (a, b) = dfs(&graph, nxt);
        if (a + b) % 2 == 0 {
            even.push((a, b));
        } else {
            odd.push((a, b));
        }
    }
    let mut ret = (1, 0);
    for (a, b) in even {
        if a <= b || odd.len() % 2 == 0 {
            ret.0 += a;
            ret.1 += b;
        } else {
            ret.1 += a;
            ret.0 += b;
        }
    }
    odd.sort_by(|(a, b), (c, d)| (a - b).cmp(&(c - d)));
    for (i, (a, b)) in odd.into_iter().enumerate() {
        if i % 2 == 0 {
            ret.0 += a;
            ret.1 += b;
        } else {
            ret.1 += a;
            ret.0 += b;
        }
    }

    ret
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n = sc.read::<usize>();
    let mut graph = vec![vec![]; n];
    for i in 1..n {
        let p = sc.usize0();
        graph[p].push(i);
    }
    let (ans, _) = dfs(&graph, 0);
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