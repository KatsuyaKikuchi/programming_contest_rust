use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

struct Node {
    edge: Vec<usize>
}

impl Node {
    fn new() -> Self {
        Node {
            edge: Vec::new(),
        }
    }

    fn add_egde(&mut self, x: usize) {
        self.edge.push(x);
    }
}

fn main()
{
    input! {
    n:usize,
    (s,t):(Usize1,Usize1),
    m:usize,
    edge:[(Usize1,Usize1);m]
    }

    let mut v = (0..n).map(|_| Node::new()).collect::<Vec<Node>>();
    for &(a, b) in edge.iter() {
        v[a].add_egde(b);
        v[b].add_egde(a);
    }

    let _mod = 1000000007i64;
    let mut ans = (0..n)
        .map(|i| if i == s { 1 } else { 0 })
        .collect::<Vec<i64>>();
    let mut dist = (0..n)
        .map(|i| if i == s { 0 } else { 1 << 30 })
        .collect::<Vec<u32>>();
    let mut q = VecDeque::new();
    q.push_back((s, 0));
    while let Some((idx, cost)) = q.pop_front() {
        let cnt = ans[idx];
        for &nxt in v[idx].edge.iter() {
            if dist[nxt] < cost + 1 {
                continue;
            }
            ans[nxt] = (ans[nxt] + cnt) % _mod;
            if dist[nxt] <= cost + 1 {
                continue;
            }
            dist[nxt] = cost + 1;
            q.push_back((nxt, cost + 1));
        }
    }
    println!("{}", ans[t]);
}