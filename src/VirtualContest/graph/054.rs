use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main()
{
    input! {
    (n,m,k):(usize,usize,i64),
    t:[i64;n-2],
    edge:[(Usize1,Usize1,i64,i64);m],
    }

    let mut v = vec![vec![]; n];
    for (a, b, c, d) in edge {
        v[a].push((b, c, d));
        v[b].push((a, c, d));
    }

    let inf = 1i64 << 60;
    let mut dist = vec![inf; n];
    dist[0] = 0;

    let mut q = BinaryHeap::new();
    q.push(Reverse((0, 0)));
    while let Some(Reverse((dst, idx))) = q.pop() {
        if dist[idx] < dst {
            continue;
        }

        for &(nxt, cst, d) in v[idx].iter() {
            let mut cost = dst;
            if idx > 0 && idx < n - 1 {
                cost += t[idx - 1];
            }
            cost += (d - (cost % d)) % d + cst;
            if dist[nxt] <= cost {
                continue;
            }
            dist[nxt] = cost;
            q.push(Reverse((cost, nxt)));
        }
    }

    let ans = if dist[n - 1] > k {
        -1
    } else {
        dist[n - 1]
    };

    println!("{}", ans);
}