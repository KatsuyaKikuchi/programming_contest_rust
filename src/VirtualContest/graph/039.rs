use proconio::input;
use proconio::marker::Usize1;
use std::collections::{VecDeque, BinaryHeap};
use std::cmp::max;

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

fn main()
{
    input! {
    n:usize,
    parent:[Usize1;n-1],
    }

    let mut tree = RootedTree::new(n, 0);
    for (i, &p) in parent.iter().enumerate() {
        tree.add_edge(i + 1, p, 1);
    }
    tree.build();

    let mut depth = vec![0; n];
    let mut used = vec![false; n];
    let mut q = BinaryHeap::new();
    for (idx, depth) in tree.node.iter().enumerate()
        .filter(|(_, n)| n.edge.len() == 0).map(|(i, n)| (i, n.depth)) {
        q.push((depth, idx));
        used[idx] = true;
    }

    while let Some((_, idx)) = q.pop() {
        let mut v = Vec::new();
        for &(child, _) in tree.node[idx].edge.iter() {
            v.push(depth[child]);
        }
        v.sort();
        v.reverse();
        for (i, &value) in v.iter().enumerate() {
            depth[idx] = max(depth[idx], i + 1 + value);
        }

        let p = tree.node[idx].parent;
        if used[p] {
            continue;
        }
        used[p] = true;
        q.push((tree.node[p].depth, p));
    }

    println!("{}", depth[0]);
}