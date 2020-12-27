use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

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

fn main()
{
    input! {
    (n,m):(usize,usize),
    a:[i64;n],
    edge:[(usize,usize);m],
    }

    let mut uf = UnionFind::new(n);
    for (a, b) in edge {
        uf.unit(a, b);
    }

    let groups = uf.groups();
    let mut count = (groups.len() as i32 - 1) * 2;
    if count > n as i32 {
        println!("Impossible");
        return;
    }

    let mut ans = 0;
    let mut q = BinaryHeap::new();

    for g in groups {
        let mut v = vec![0; g.len()];
        for (i, &x) in g.iter().enumerate() {
            v[i] = a[x];
        }
        v.sort();
        if count > 0 {
            ans += v[0];
        }
        count -= 1;
        for i in 1..v.len() {
            q.push(Reverse(v[i]));
        }
    }

    while count > 0 {
        if let Some(Reverse(val)) = q.pop() {
            ans += val;
        }
        count -= 1;
    }

    println!("{}", ans);
}