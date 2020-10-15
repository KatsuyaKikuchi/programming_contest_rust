use proconio::input;
use std::collections::{VecDeque, BinaryHeap};
use proconio::marker::Usize1;

struct Node {
    edge: Vec<(usize, i64)>,
    parent: usize,
    depth: i64,
}

struct RootedTree {
    node: Vec<Node>,
    root: usize,
}

impl Node {
    fn new(parent: usize) -> Self {
        Node {
            edge: Vec::new(),
            parent: parent,
            depth: 0,
        }
    }
}

#[allow(dead_code)]
impl RootedTree {
    fn new(n: usize, r: usize) -> Self {
        RootedTree {
            node: (0..n).map(|i| Node::new(i)).collect(),
            root: r,
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