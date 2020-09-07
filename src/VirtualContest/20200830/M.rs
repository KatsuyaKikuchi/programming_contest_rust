use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;
use std::cmp::{Reverse, max};

struct Node {
    edge: Vec<(usize, i64)>,
}

impl Node {
    fn new() -> Self {
        Node {
            edge: Vec::new(),
        }
    }
}

fn dijkstra(s: usize, node: &Vec<Node>) -> Vec<i64> {
    let n = node.len();
    let mut q = BinaryHeap::new();
    let inf = 1i64 << 60;
    let mut cost = vec![inf; n];
    q.push(Reverse((0, s)));
    cost[s] = 0;
    while let Some(Reverse((c, idx))) = q.pop() {
        if cost[idx] < c {
            continue;
        }
        for &(nxt, cst) in node[idx].edge.iter() {
            let nxt_cst = cst + c;
            if cost[nxt] <= nxt_cst {
                continue;
            }
            cost[nxt] = nxt_cst;
            q.push(Reverse((nxt_cst, nxt)));
        }
    }

    cost
}

fn main()
{
    input! {
    n:usize,
    m:usize,
    t:i64,
    v:[i64;n],
    edge:[(Usize1,Usize1,i64);m]
    }

    let mut to = (0..n).map(|_| Node::new()).collect::<Vec<Node>>();
    let mut from = (0..n).map(|_| Node::new()).collect::<Vec<Node>>();
    for (a, b, c) in edge {
        to[a].edge.push((b, c));
        from[b].edge.push((a, c));
    }

    let to_cost = dijkstra(0, &to);
    let from_cost = dijkstra(0, &from);
    let mut ans = 0;
    for i in 0..n {
        let p = max(0, t - (to_cost[i] + from_cost[i]));
        ans = max(ans, p * v[i]);
    }

    println!("{}", ans);
}