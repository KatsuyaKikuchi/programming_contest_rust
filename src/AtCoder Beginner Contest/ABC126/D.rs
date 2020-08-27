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
    v:[(Usize1,Usize1,i64);n-1],
    }

    let mut tree = RootedTree::new(n, 0);
    for &(a, b, w) in v.iter() {
        tree.add_edge(a, b, w);
    }

    tree.build();

    for _n in tree.node.iter() {
        println!("{}", _n.depth % 2);
    }
}