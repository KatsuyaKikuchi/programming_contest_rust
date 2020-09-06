use proconio::input;
use proconio::marker::Usize1;
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
    m:usize,
    edge:[(Usize1,Usize1,i64);m],
    }

    let mut uf = UnionFind::new(n);

    let mut ans = Vec::new();
    let mut q = BinaryHeap::new();
    for (i, (a, b, c)) in edge.into_iter().enumerate() {
        q.push((c, (a, b), i));
    }

    while let Some((_c, (a, b), idx)) = q.pop() {
        if uf.same(a, b) {
            continue;
        }
        ans.push(idx + 1);
        uf.unit(a, b);
    }

    ans.sort();
    for i in ans {
        println!("{}", i);
    }
}