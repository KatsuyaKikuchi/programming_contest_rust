use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

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

fn solve(node: Vec<(i64, i64, u32)>) -> f64 {
    let n = node.len();
    let mut uf = UnionFind::new(n);
    let mut q = BinaryHeap::new();
    let mut cost = 0.0;
    for i in 0..n {
        for j in i + 1..n {
            let (dx, dy) = (node[i].0 - node[j].0, node[i].1 - node[j].1);
            let mut len = dx * dx + dy * dy;
            if node[i].2 != node[j].2 {
                len *= 100;
            }

            q.push(Reverse((len, i, j)));
        }
    }

    while let Some(Reverse((len, a, b))) = q.pop() {
        if uf.same(a, b) {
            continue;
        }
        cost += (len as f64).sqrt();
        uf.unit(a, b);
    }

    cost
}

fn main()
{
    input! {
    n:usize,
    m:usize,
    node0:[(i64,i64,u32);n],
    node1:[(i64,i64,u32);m],
    }

    let mut ans = 1000000000000.0;
    for b in 0..(1u32 << m) {
        let node2 = node1.iter().enumerate()
            .filter(|&(idx, _g)| ((b >> idx) & 1) > 0)
            .into_iter()
            .map(|(_idx, g)| g.clone()).collect::<Vec<(i64, i64, u32)>>();

        let node = node0.clone().into_iter().chain(node2).collect::<Vec<(i64, i64, u32)>>();
        let cost = solve(node);
        if ans > cost {
            ans = cost;
        }
    }

    println!("{:.9}", ans);
}