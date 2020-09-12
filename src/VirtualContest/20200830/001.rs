use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;
use std::cmp::max;

struct Node {
    edge: Vec<(usize, usize)>,
}

struct Tree {
    nodes: Vec<Node>,
}

impl Node {
    fn new() -> Self {
        Node {
            edge: Vec::new(),
        }
    }
}

impl Tree {
    fn new(n: usize) -> Self {
        Tree {
            nodes: (0..n).map(|_| Node::new()).collect(),
        }
    }

    fn add_edge(&mut self, a: usize, b: usize, i: usize) {
        self.nodes[a].edge.push((b, i));
        self.nodes[b].edge.push((a, i));
    }
}

fn main()
{
    input! {
    n:usize,
    e:[(Usize1,Usize1);n-1],
    }

    let mut tree = Tree::new(n);
    for (i, &(a, b)) in e.iter().enumerate() {
        tree.add_edge(a, b, i);
    }

    let mut ans = vec![0; n - 1];
    let mut mx = 0;
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    while let Some((t, col)) = q.pop_back() {
        let mut m = 0;
        for &(node, edge) in tree.nodes[t].edge.iter() {
            if ans[edge] > 0 {
                continue;
            }
            m += 1;
            if m == col {
                m += 1;
            }
            ans[edge] = m;
            q.push_back((node, m));
        }
        mx = max(mx, m );
    }

    println!("{}", mx);
    for &i in ans.iter() {
        println!("{}", i);
    }
}