use proconio::input;
use proconio::marker::Usize1;
use std::collections::{VecDeque, BinaryHeap};
use std::cmp::Reverse;

fn main()
{
    input! {
    n:usize,
    m:usize,
    k:usize,
    s:i64,
    (x,y):(i64,i64),
    v:[Usize1;k],
    edge:[(Usize1,Usize1);m]
    }

    let inf = 1i64 << 60;
    let mut dist = vec![inf; n];
    let mut q = VecDeque::new();
    for i in v {
        dist[i] = 0;
        q.push_back(i);
    }

    let mut node = vec![vec![]; n];
    for (a, b) in edge {
        node[a].push(b);
        node[b].push(a);
    }

    while let Some(idx) = q.pop_front() {
        let cst = dist[idx] + 1;
        for &nxt in node[idx].iter() {
            if dist[nxt] <= cst {
                continue;
            }
            dist[nxt] = cst;
            q.push_back(nxt);
        }
    }

    let cost = (0..n)
        .map(|i| {
            if i == 0 || i == n - 1 {
                0
            } else if dist[i] == 0 {
                inf
            } else if dist[i] <= s {
                y
            } else {
                x
            }
        }).collect::<Vec<i64>>();

    let mut ans = vec![inf; n];
    ans[0] = 0;
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, 0)));
    while let Some(Reverse((cst, idx))) = q.pop() {
        if ans[idx] < cst {
            continue;
        }
        for &nxt in node[idx].iter() {
            let cst = cst + cost[nxt];
            if ans[nxt] <= cst {
                continue;
            }
            ans[nxt] = cst;
            q.push(Reverse((cst, nxt)));
        }
    }

    println!("{}", ans[n - 1]);
}