use proconio::input;
use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).map(|i| i).collect(),
            rank: vec![0; n],
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

    fn unit(&mut self, n: usize, m: usize) {
        let n = self.find(n);
        let m = self.find(m);
        if n == m {
            return;
        }
        if self.rank[n] > self.rank[m] {
            self.parent[m] = n;
        } else {
            self.parent[n] = m;
            if self.rank[n] == self.rank[m] {
                self.rank[n] += self.rank[m];
            }
        }
    }
}

fn main()
{
    input! {
    n:usize,
    mut g:[(i64,i64);n],
    }
    let mut uf = UnionFind::new(n);
    let mut v = Vec::new();
    for (i, &(a, b)) in g.iter().enumerate() {
        v.push((a, b, i));
    }
    v.sort();
    let mut q = BinaryHeap::new();
    for i in 0..v.len() - 1 {
        let (a, b) = (v[i].2, v[i + 1].2);
        let (x0, y0) = (v[i].0, v[i].1);
        let (x1, y1) = (v[i + 1].0, v[i + 1].1);
        let dist = min((x0 - x1).abs(), (y0 - y1).abs());
        q.push(Reverse((dist, a, b)));
    }
    v.sort_by(|(_, y0, _), (_, y1, _)| y0.cmp(y1));
    for i in 0..v.len() - 1 {
        let (a, b) = (v[i].2, v[i + 1].2);
        let (x0, y0) = (v[i].0, v[i].1);
        let (x1, y1) = (v[i + 1].0, v[i + 1].1);
        let dist = min((x0 - x1).abs(), (y0 - y1).abs());
        q.push(Reverse((dist, a, b)));
    }

    let mut ans = 0;

    while let Some(Reverse((d, a, b))) = q.pop() {
        if uf.same(a, b) {
            continue;
        }
        uf.unit(a, b);
        ans += d;
    }

    println!("{}", ans);
}