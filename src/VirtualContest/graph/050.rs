use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<i32>,
    size: usize,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).map(|i| i).collect(),
            rank: vec![1; size],
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

fn main() {
    input! {
    (n,k,l):(usize,usize,usize),
    v0:[(Usize1,Usize1);k],
    v1:[(Usize1,Usize1);l],
    }

    let mut uf0 = UnionFind::new(n);
    let mut uf1 = UnionFind::new(n);

    for (a, b) in v0 {
        uf0.unit(a, b);
    }
    for (a, b) in v1 {
        uf1.unit(a, b);
    }

    let mut m = HashMap::new();

    for i in 0..n {
        let x = (uf0.find(i), uf1.find(i));
        let val = m.entry(x).or_insert(0);
        *val += 1;
    }

    for i in 0..n {
        let x = (uf0.find(i), uf1.find(i));
        let val = m.get(&x).unwrap();
        print!("{} ", val);
    }
    println!("");
}