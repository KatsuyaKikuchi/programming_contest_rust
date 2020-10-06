use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;
use std::cmp::{Reverse, min};

fn main()
{
    input! {
    (n,m):(usize,usize),
    c:i64,
    edge:[(Usize1,Usize1,i64);m],
    }

    let mut ans = 0;
    let mut v = vec![vec![]; n];
    for (a, b, c) in edge {
        v[a].push((b, c));
        v[b].push((a, c));
        ans += c;
    }

    let mut q = BinaryHeap::new();
    let inf = 1i64 << 60;
    let mut dist = vec![inf; n];
    dist[0] = 0;
    q.push(Reverse((0, 0)));

    while let Some(Reverse((dst, idx))) = q.pop() {
        if dist[idx] < dst {
            continue;
        }

        for &(nxt, cst) in v[idx].iter() {
            let cost = dst + cst;
            if dist[nxt] <= cost {
                continue;
            }
            dist[nxt] = cost;
            q.push(Reverse((cost, nxt)));
        }
    }

    let mut dist = dist.iter().enumerate().map(|(i, &c)| (c, i)).collect::<Vec<(i64, usize)>>();
    dist.sort();

    let mut sum = ans;
    let mut used = vec![false; n];
    for (dst, idx) in dist {
        for &(nxt, c) in v[idx].iter() {
            if used[nxt] {
                sum -= c;
            }
        }
        used[idx] = true;
        ans = min(ans, c * dst + sum);
    }

    println!("{}", ans);
}