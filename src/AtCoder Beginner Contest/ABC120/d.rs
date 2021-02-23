pub struct UnionFind {
    n: usize,
    parent_or_size: Vec<i32>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            n: size,
            parent_or_size: vec![-1; size],
        }
    }

    pub fn find(&mut self, idx: usize) -> usize {
        assert!(idx < self.n);
        if self.parent_or_size[idx] < 0 {
            return idx;
        }
        self.parent_or_size[idx] = self.find(self.parent_or_size[idx] as usize) as i32;
        self.parent_or_size[idx] as usize
    }

    pub fn same(&mut self, idx0: usize, idx1: usize) -> bool {
        self.find(idx0) == self.find(idx1)
    }

    pub fn unit(&mut self, idx0: usize, idx1: usize) -> usize {
        let (idx0, idx1) = (self.find(idx0), self.find(idx1));
        if idx0 == idx1 {
            return idx0;
        }
        if -self.parent_or_size[idx0] < -self.parent_or_size[idx1] {
            return self.unit(idx1, idx0);
        }
        self.parent_or_size[idx0] += self.parent_or_size[idx1];
        self.parent_or_size[idx1] = idx0 as i32;
        idx0
    }

    pub fn size(&mut self, idx: usize) -> usize {
        let idx = self.find(idx);
        -self.parent_or_size[idx] as usize
    }


    pub fn group(&mut self) -> Vec<Vec<usize>> {
        let mut parent_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            parent_buf[i] = self.find(i);
            group_size[parent_buf[i]] += 1;
        }
        let mut result = vec![Vec::new(); self.n];
        for i in 0..self.n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            result[parent_buf[i]].push(i);
        }

        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let n: usize = sc.read();
    let m: usize = sc.read();
    let mut edge = vec![];
    for _ in 0..m {
        let (a, b) = (sc.usize0(), sc.usize0());
        edge.push((a, b));
    }
    let mut ans = vec![n * (n - 1) / 2];
    edge.reverse();
    edge.pop();
    let mut uf = UnionFind::new(n);
    for (a, b) in edge {
        let nxt = if uf.same(a, b) {
            ans.last().unwrap().clone()
        } else {
            let diff = uf.size(a) * uf.size(b);
            uf.unit(a, b);
            ans.last().unwrap().clone() - diff
        };
        ans.push(nxt);
    }
    ans.reverse();
    for v in ans {
        println!("{}", v);
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