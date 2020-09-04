use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use proconio::marker::Usize1;

struct Node {
    edge: Vec<(usize, i64)>,
}

struct Graph
{
    node: Vec<Node>
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            node: (0..n).map(|_| Node::new()).collect(),
        }
    }

    fn add_edge(&mut self, a: usize, b: usize, c: i64) {
        self.node[a].edge.push((b, c));
        self.node[b].edge.push((a, c));
    }

    fn cost(&self, a: usize, b: usize) -> i64 {
        let mut cost = vec![1i64 << 50; self.node.len()];
        cost[a] = 0;
        let mut q = BinaryHeap::new();
        q.push(Reverse((0, a)));

        while let Some(Reverse((dist, idx))) = q.pop() {
            if dist > cost[idx] {
                continue;
            }
            for &(nxt, cst) in self.node[idx].edge.iter() {
                let c = dist + cst;
                if cost[nxt] <= c {
                    continue;
                }
                cost[nxt] = c;
                q.push(Reverse((c, nxt)));
            }
        }

        if cost[b] < (1i64 << 50) {
            cost[b]
        } else {
            -1
        }
    }
}

impl Node {
    fn new() -> Self
    {
        Node {
            edge: Vec::new(),
        }
    }
}

fn main()
{
    input! {
    n:usize,
    k:usize,
    }

    let mut graph = Graph::new(n);

    for _ in 0..k {
        input! {
        x:i32
        }
        if x == 1 {
            input! {
            a:Usize1,
            b:Usize1,
            c:i64,
            }
            graph.add_edge(a, b, c);
        } else {
            input! {
            a:Usize1,
            b:Usize1,
            }
            println!("{}", graph.cost(a, b));
        }
    }
}