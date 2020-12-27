use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;
use std::cmp::{Reverse, max};

fn main()
{
    input! {
    (n,m,k):(usize,usize,usize),
    edge:[(Usize1,Usize1,i64);m],
    s:[Usize1;k],
    }

    let mut v = vec![vec![]; n];
    for &(a, b, c) in edge.iter() {
        v[a].push((b, c));
        v[b].push((a, c));
    }

    let inf = 1i64 << 60;
    let mut ans = 0;
    let mut dist = vec![inf; n];

    let mut q = BinaryHeap::new();
    for t in s {
        q.push(Reverse((0, t)));
        dist[t] = 0;
    }

    while let Some(Reverse((dst, idx))) = q.pop() {
        if dist[idx] < dst {
            continue;
        }
        ans = max(ans, dst);

        for &(nxt, c) in v[idx].iter() {
            if dist[nxt] <= dist[idx] && dist[nxt] + c != dist[idx] {
                let cost = dist[nxt] + (dist[idx] - dist[nxt] + c + 1) / 2;
                ans = max(ans, cost);
                continue;
            }
            let cost = c + dst;
            if dist[nxt] < cost {
                continue;
            }
            dist[nxt] = cost;
            q.push(Reverse((cost, nxt)));
        }
    }

    println!("{}", ans);
}