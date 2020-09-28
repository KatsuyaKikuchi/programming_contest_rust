use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;
use std::cmp::{Reverse, min};

fn main()
{
    input! {
    n:usize,
    m:usize,
    (s,t):(Usize1,Usize1),
    edge:[(Usize1,Usize1,i64,i64);m],
    }
    let inf = 10i64.pow(15);
    let mut v = vec![vec![]; n];
    for (a, b, c, d) in edge {
        v[a].push((b, c, d));
        v[b].push((a, c, d));
    }

    let mut len_s = vec![inf; n];
    let mut len_t = vec![inf; n];

    len_s[s] = 0;
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, s)));
    while let Some(Reverse((dist, idx))) = q.pop() {
        if len_s[idx] < dist {
            continue;
        }
        for &(nxt, cost, _) in v[idx].iter() {
            let dist = dist + cost;
            if len_s[nxt] <= dist {
                continue;
            }
            len_s[nxt] = dist;
            q.push(Reverse((dist, nxt)));
        }
    }
    len_t[t] = 0;
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, t)));
    while let Some(Reverse((dist, idx))) = q.pop() {
        if len_t[idx] < dist {
            continue;
        }
        for &(nxt, _, cost) in v[idx].iter() {
            let dist = dist + cost;
            if len_t[nxt] <= dist {
                continue;
            }
            len_t[nxt] = dist;
            q.push(Reverse((dist, nxt)));
        }
    }

    let mut ans = vec![inf; n + 1];
    for i in (0..n).rev() {
        ans[i] = min(ans[i + 1], len_s[i] + len_t[i]);
    }

    for i in 0..n {
        println!("{}", inf - ans[i]);
    }
}