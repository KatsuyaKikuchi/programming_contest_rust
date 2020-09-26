use proconio::input;
use std::collections::{VecDeque, BinaryHeap};
use std::cmp::max;

struct Node {
    edge: Vec<(usize, i64)>,
    parent: usize,
    depth: i64,
    size: usize,
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
            size: 1,
        }
    }
}

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
    n:usize,
    parent:[usize;n-1],
    }

    let mut tree = RootedTree::new(n, 0);
    for (i, &p) in parent.iter().enumerate() {
        tree.add_edge(i + 1, p, 1);
    }
    tree.build();

    let mut ans = vec![0; n];
    let mut used = vec![false; n];

    let mut q = BinaryHeap::new();
    for i in 0..n {
        if tree.node[i].edge.len() == 0 {
            q.push((tree.node[i].depth, i));
        }
    }

    while let Some((_, idx)) = q.pop()
    {
        if used[idx] {
            continue;
        }
        used[idx] = true;
        ans[idx] = match tree.node[idx].edge.iter()
            .map(|&(i, _)| tree.node[i].size)
            .max() {
            Some(t) => t,
            None => 0,
        };
        tree.node[idx].size += tree.node[idx].edge.iter()
            .map(|&(i, _)| tree.node[i].size)
            .sum::<usize>();

        ans[idx] = max(ans[idx], n - tree.node[idx].size);

        let p = tree.node[idx].parent;
        if p == idx {
            continue;
        }
        q.push((tree.node[p].depth, p));
    }

    for a in ans {
        println!("{}", a);
    }
}