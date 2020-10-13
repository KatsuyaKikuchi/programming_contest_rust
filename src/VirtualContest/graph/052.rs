use proconio::input;
use std::collections::VecDeque;
use proconio::marker::Usize1;

struct Node {
    edge: Vec<(usize, i64)>,
    parent: usize,
    depth: i64,
    doubling: Vec<usize>,
}

struct RootedTree {
    node: Vec<Node>,
    root: usize,
    doubling_size: usize,
}

impl Node {
    fn new(parent: usize) -> Self {
        Node {
            edge: Vec::new(),
            parent: parent,
            depth: 0,
            doubling: Vec::new(),
        }
    }
}

#[allow(dead_code)]
impl RootedTree {
    fn new(n: usize, r: usize) -> Self {
        RootedTree {
            node: (0..n).map(|i| Node::new(i)).collect(),
            root: r,
            doubling_size: 60,
        }
    }

    fn add_edge(&mut self, v0: usize, v1: usize, weight: i64) {
        self.node[v0].edge.push((v1, weight));
        self.node[v1].edge.push((v0, weight));
    }

    fn build(&mut self) {
        let mut q: VecDeque<(usize, usize, i64)> = VecDeque::new();
        q.push_back((self.root, self.root, 0));

        while let Some((i, p, d)) = q.pop_back() {
            self.node[i].depth = d;
            self.node[i].parent = p;
            if let Some(idx) = self.node[i].edge.iter()
                .position(|&(i, _)| i == p) {
                self.node[i].edge.remove(idx);
            }

            for &(nxt, weight) in self.node[i].edge.iter() {
                let depth = weight + d;
                q.push_back((nxt, i, depth));
            }
        }

        for i in 0..self.node.len() {
            if i == self.root {
                self.node[i].doubling.push(i);
            } else {
                let p = self.node[i].parent;
                self.node[i].doubling.push(p);
            }
        }

        for i in 0..self.doubling_size {
            for j in 0..self.node.len() {
                let mut p = self.node[j].doubling[i];
                p = self.node[p].doubling[i];
                self.node[j].doubling.push(p);
            }
        }
    }

    // weight != 1 だと多分動かない
    fn lca(&self, a: usize, b: usize) -> usize {
        let (mut a, mut b) = if self.node[a].depth > self.node[b].depth {
            (b, a)
        } else {
            (a, b)
        };

        let diff = self.node[b].depth - self.node[a].depth;
        for i in 0..self.doubling_size {
            if ((diff >> i) & 1) == 1 {
                b = self.node[b].doubling[i];
            }
        };

        if a == b {
            return a;
        }

        for i in (0..self.doubling_size).rev() {
            if self.node[a].doubling[i] == self.node[b].doubling[i] {
                continue;
            }
            a = self.node[a].doubling[i];
            b = self.node[b].doubling[i];
        };

        self.node[a].doubling[0]
    }

    fn len(&self, a: usize, b: usize) -> i64 {
        let p = self.lca(a, b);
        self.node[a].depth + self.node[b].depth - 2 * (self.node[p].depth)
    }
}

// vの中から、valより大きい値が初めて出てくるindexを返す
// vは昇順ソートされている前提
fn upper_bound(v: &Vec<i64>, val: i64) -> usize {
    let mut ret = v.len() as i32;
    let mut ng = -1;
    while (ret - ng).abs() > 1 {
        let mid = ((ret + ng) / 2) as usize;
        if v[mid] >= val {
            ret = mid as i32;
        } else {
            ng = mid as i32;
        }
    }
    ret as usize
}

fn dfs(tree: &RootedTree, idx: usize, v: &mut Vec<i64>, ans: &mut Vec<usize>, a: &Vec<i64>) {
    let len = upper_bound(v, a[idx]);
    let prev = v[len];
    v[len] = a[idx];
    let inf = 1i64 << 60;
    ans[idx] = upper_bound(v, inf);
    for &(nxt, _) in tree.node[idx].edge.iter() {
        dfs(tree, nxt, v, ans, a);
    }
    v[len] = prev;
}

fn main() {
    input! {
    n:usize,
    a:[i64;n],
    edge:[(Usize1,Usize1);n-1],
    }

    let mut tree = RootedTree::new(n, 0);
    for (a, b) in edge {
        tree.add_edge(a, b, 1);
    }
    tree.build();

    let inf = 1i64 << 60;
    let mut v = vec![inf; n + 5];
    let mut ans = vec![n + 1; n];

    dfs(&tree, 0, &mut v, &mut ans, &a);

    for val in ans {
        println!("{}", val);
    }
}