use proconio::input;
use proconio::marker::Usize1;
use std::collections::{BinaryHeap, VecDeque};
use std::cmp::Reverse;

fn main() {
    input! {
    n:usize,
    k:usize,
    v:[(i64,i64);n],
    edge:[(Usize1,Usize1);k],
    }

    let mut node = vec![vec![]; n];
    for (a, b) in edge {
        node[a].push(b);
        node[b].push(a);
    }

    let inf = 1i64 << 60;
    let mut cost = vec![inf; n];
    let mut hq = BinaryHeap::new();
    hq.push(Reverse((0, 0)));
    while let Some(Reverse((cst, idx))) = hq.pop() {
        if cost[idx] < cst {
            continue;
        }
        let cst = cst + v[idx].0;
        let r = v[idx].1;
        let mut dist = vec![inf; n];
        dist[idx] = 0;
        let mut q = VecDeque::new();
        q.push_back(idx);
        while let Some(idx) = q.pop_front() {
            let dst = dist[idx] + 1;
            if r < dst {
                continue;
            }
            for &nxt in node[idx].iter() {
                if dist[nxt] <= dst {
                    continue;
                }
                dist[nxt] = dst;
                q.push_back(nxt);
                if cst < cost[nxt] {
                    cost[nxt] = cst;
                    hq.push(Reverse((cst, nxt)));
                }
            }
        }
    }

    println!("{}", cost[n - 1]);
}