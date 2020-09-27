use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;
use std::cmp::max;

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
    a:Usize1,
    b:Usize1,
    edge:[(Usize1,Usize1);n-1],
    }

    let mut tree = RootedTree::new(n, b);
    for (a, b) in edge {
        tree.add_edge(a, b, 1);
    }
    tree.build();

    let mut q = VecDeque::new();
    q.push_back((a, 0));
    let inf = 1i64 << 60;
    let mut v = vec![inf; n];
    v[a] = 0;
    while let Some((idx, cst)) = q.pop_front()
    {
        let cst = cst + 1;
        let p = tree.node[idx].parent;
        if v[p] > cst {
            v[p] = cst;
            q.push_back((p, cst));
        }
        for &(nxt, _) in tree.node[idx].edge.iter()
        {
            if v[nxt] <= cst {
                continue;
            }
            v[nxt] = cst;
            q.push_back((nxt, cst));
        }
    }

    let mut ans = 0;
    for i in 0..n {
        let node = &tree.node[i];
        if node.edge.len() == 0 {
            continue;
        }
        if v[i] > node.depth {
            continue;
        }
        ans = max(ans, node.depth);
    }
    println!("{}", ans);
}