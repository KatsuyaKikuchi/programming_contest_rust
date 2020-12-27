use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;
use std::cmp::{Reverse, min};

fn main()
{
    input! {
    (n,m,c):(usize,usize,i64),
    edge:[(Usize1,Usize1,i64);m]
    }
    let mut v = vec![Vec::new(); n];
    for (idx, &(a, b, c)) in edge.iter().enumerate() {
        v[a].push((b, c, idx));
        v[b].push((a, c, idx));
    }
    let mut cost = edge.iter().fold(0, |c, &(_, _, d)| c + d);
    let mut ans = cost;
    let mut q = BinaryHeap::new();
    let mut seen = vec![false; m];
    q.push(Reverse((0, 0)));
    let inf = 1i64 << 60;
    let mut dist = vec![inf; n];
    dist[0] = 0;
    while let Some(Reverse((cst, idx))) = q.pop() {
        if dist[idx] < cst {
            continue;
        }
        for &(nxt, d, i) in v[idx].iter() {
            if dist[nxt] > cst || seen[i] {
                continue;
            }
            seen[i] = true;
            cost -= d;
        }
        ans = min(ans, cost + cst * c);
        for &(nxt, d, _) in v[idx].iter() {
            let cst = cst + d;
            if dist[nxt] <= cst {
                continue;
            }
            dist[nxt] = cst;
            q.push(Reverse((cst, nxt)));
        }
    }
    println!("{}", ans);
}