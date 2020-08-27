use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashSet;

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
    v:[(Usize1,Usize1,i32);m],
    }

    let mut uf = UnionFind::new(n);
    for &(a, b, _) in v.iter() {
        uf.unit(a, b);
    }

    let mut id: HashSet<usize> = (0..n).into_iter().map(|i| uf.find(i)).collect();
    println!("{}", id.iter().count());
}