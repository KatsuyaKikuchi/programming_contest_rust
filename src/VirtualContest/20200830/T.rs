use proconio::input;
use std::collections::VecDeque;
use std::cmp::min;

struct Node {
    edge: Vec<(usize, i64)>,
    parent: usize,
    depth: i64,
    cost: i32,
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
            cost: 1 << 30,
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
    m:usize,
    p:[usize;n-1],
    v:[(usize,i32);m]
    }

    let mut tree = RootedTree::new(n, 0);
    for (i, &par) in p.iter().enumerate() {
        tree.add_edge(par, i + 1, 1);
    }
    tree.build();
    let mut q = VecDeque::new();
    for (n, c) in v {
        q.push_back(n);
        tree.node[n].cost = c;
    }

    let mut count = vec![0; n];

    while let Some(idx) = q.pop_front() {
        let parent = tree.node[idx].parent;
        if parent == 0 {
            continue;
        }
        tree.node[parent].cost = min(tree.node[idx].cost, tree.node[parent].cost);
        count[parent] += 1;
        if count[parent] == tree.node[parent].edge.len() {
            q.push_back(parent);
        }
    }

    tree.node[0].cost = 0;
    let mut ans = 0;
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(idx) = q.pop_front() {
        for i in 0..tree.node[idx].edge.len() {
            let nxt = tree.node[idx].edge[i].0;
            q.push_back(nxt);
            ans += tree.node[nxt].cost - tree.node[idx].cost;
        }
    }
    println!("{}", ans);
}