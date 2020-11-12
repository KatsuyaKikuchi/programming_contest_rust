use proconio::input;
use proconio::marker::Usize1;
use std::cmp::max;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<i32>,
    count: Vec<usize>,
    size: usize,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).map(|i| i).collect(),
            rank: vec![1; size],
            count: vec![1; size],
            size: size,
        }
    }

    fn find(&mut self, n: usize) -> usize {
        if self.parent[n] == n {
            return n;
        };
        self.parent[n] = self.find(self.parent[n]);
        self.parent[n]
    }

    fn same(&mut self, n: usize, m: usize) -> bool {
        self.find(n) == self.find(m)
    }

    fn unit(&mut self, n: usize, m: usize) -> usize {
        let n = self.find(n);
        let m = self.find(m);
        if n == m {
            return n;
        }
        if self.rank[n] > self.rank[m] {
            self.parent[m] = n;
            self.count[n] += self.count[m];
            n
        } else {
            self.parent[n] = m;
            self.count[m] += self.count[n];
            if self.rank[n] == self.rank[m] {
                self.rank[n] += self.rank[m];
            }
            m
        }
    }

    fn size(&mut self, n: usize) -> usize {
        let n = self.find(n);
        self.count[n]
    }

    fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut parent_buf = vec![0; self.size];
        let mut group_size = vec![0; self.size];
        for i in 0..self.size {
            parent_buf[i] = self.find(i);
            group_size[parent_buf[i]] += 1;
        }

        let mut ret = vec![Vec::new(); self.size];
        for i in 0..self.size {
            ret[i].reserve(group_size[i]);
        }

        for i in 0..self.size {
            ret[parent_buf[i]].push(i);
        }

        ret
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect()
    }
}

fn main()
{
    input! {
    (n,m,w):(usize,usize,usize),
    t:[(usize,i64);n],
    p:[(Usize1,Usize1);m]
    }

    let mut uf = UnionFind::new(n);
    for (a, b) in p {
        uf.unit(a, b);
    }

    let group = uf.groups();
    let mut v = Vec::new();
    for g in group {
        let (mut weight, mut value) = (0, 0);
        for i in g {
            weight += t[i].0;
            value += t[i].1;
        }
        v.push((weight, value));
    }

    let n = v.len();
    let mut dp = vec![vec![0; w + 1]; n + 1];
    for (i, &(a, b)) in v.iter().enumerate() {
        for j in 0..(w + 1) {
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            if j + a <= w {
                dp[i + 1][j + a] = max(dp[i + 1][j + a], dp[i][j] + b);
            }
        }
    }

    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}