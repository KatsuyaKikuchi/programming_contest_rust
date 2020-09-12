use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

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
    k:i64,
    edge:[(Usize1,Usize1);n-1],
    }

    let _mod = 10_i64.pow(9) + 7;
    let mut tree = RootedTree::new(n,0);
    for (a, b) in edge
    {
        tree.add_edge(a, b, 1);
    }

    tree.build();
    let mut ans = k;
    let mut q = VecDeque::new();
    for (i, &(nxt, _)) in tree.node[0].edge.iter().enumerate()
    {
        let i = i as i64;
        ans = (ans * (k - i - 1)) % _mod;
        q.push_back(nxt);
    }

    while let Some(idx) = q.pop_front() {
        for (i, &(nxt, _)) in tree.node[idx].edge.iter().enumerate() {
            let i = i as i64;
            ans = (ans * (k - 2 - i)) % _mod;
            q.push_back(nxt);
        }
    }

    println!("{}", ans);
}