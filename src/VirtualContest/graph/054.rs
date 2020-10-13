use proconio::input;
use std::collections::{VecDeque, BinaryHeap};
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

fn main()
{
    input! {
    (n,m):(usize,usize),
    edge:[(Usize1,Usize1);n-1],
    c:[Usize1;m],
    }

    let mut tree = RootedTree::new(n, c[0]);
    for (a, b) in edge {
        tree.add_edge(a, b, 1);
    }
    tree.build();
    let mut q = BinaryHeap::new();
    let mut pic = vec![false; n];
    for i in c {
        pic[i] = true;
    }
    for i in 0..n {
        if tree.node[i].edge.len() == 0 {
            q.push((tree.node[i].depth, i));
        }
    }

    while let Some((depth, idx)) = q.pop() {
        let mut count = if depth == 0 {
            2
        } else {
            1
        };

        for &(cld, _) in tree.node[idx].edge.iter() {
            if pic[cld] {
                count -= 1;
            }
        }

        if count < 0 {
            println!("trumpet");
            return;
        }
        let p = tree.node[idx].parent;
        if p == idx {
            continue;
        }
        pic[p] |= pic[idx];
        q.push((depth - 1, p));
    }

    println!("Yes");
}