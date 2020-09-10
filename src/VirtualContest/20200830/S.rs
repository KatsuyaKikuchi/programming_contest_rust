use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Node {
    edge: Vec<(usize, i64)>
}

struct Graph
{
    node: Vec<Node>,
}

impl Node {
    fn new() -> Self {
        Node {
            edge: Vec::new(),
        }
    }
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            node: (0..n).map(|_| Node::new()).collect::<Vec<Node>>(),
        }
    }

    fn add_edge(&mut self, a: usize, b: usize, dist: i64) {
        self.node[a].edge.push((b, dist));
    }

    fn dijkstra(&self, start: usize, goal: usize) -> i64 {
        let inf = 1i64 << 60;
        let mut cost = vec![inf; self.node.len()];
        cost[start] = 0;

        let mut q = BinaryHeap::new();
        q.push(Reverse((0, start)));

        while let Some(Reverse((cst, idx))) = q.pop() {
            if cost[idx] < cst {
                continue;
            }
            for &(nxt, dist) in self.node[idx].edge.iter() {
                let cst = dist + cst;
                if cost[nxt] <= cst {
                    continue;
                }
                cost[nxt] = cst;
                q.push(Reverse((cst, nxt)));
            }
        }

        cost[goal]
    }
}


fn main()
{
    input! {t:usize}
    for _ in 0..t {
        input! {
        n:usize,
        v:[[i64;n];n],
        }

        let mut edge = Vec::new();
        for i in 0..n {
            for j in 0..n {
                let dist = if v[i][j] < 0 {
                    1i64 << 60
                } else {
                    v[i][j]
                };
                edge.push((dist, i, j));
            }
        }
        edge.sort();

        let mut graph = Graph::new(n);
        let mut exist = true;
        for (d, a, b) in edge {
            let dist = graph.dijkstra(a, b);
            if dist < d {
                exist = false;
                break;
            }

            if dist > d {
                graph.add_edge(a, b, d);
            }
        }

        if exist {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}