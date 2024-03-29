use proconio::{input, fastout};

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

#[fastout]
fn main()
{
    input! {
    n:usize,
    q:usize,
    v:[(i32,usize,usize);q],
    }

    let mut uf = UnionFind::new(n);
    for &(p, a, b) in v.iter() {
        if p == 0 {
            uf.unit(a, b);
        } else {
            println!("{}",
                     if uf.same(a, b) {
                         "Yes"
                     } else {
                         "No"
                     });
        }
    }
}