use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main()
{
    input! {
    n:usize,
    m:usize,
    l:i64,
    edge:[(Usize1,Usize1,i64);m],
    q:usize,
    query:[(Usize1,Usize1);q],
    }

    let mut v = vec![vec![]; n];
    for (a, b, c) in edge {
        v[a].push((b, c));
        v[b].push((a, c));
    }

    let inf = 1i64 << 60;
    let mut ans = vec![vec![inf; n]; n];
    for s in 0..n {
        let mut q = BinaryHeap::new();
        q.push(Reverse((0, 0, s)));

        let mut dist = vec![(inf, inf); n];
        dist[s] = (0, 0);
        while let Some(Reverse((num, dst, idx))) = q.pop() {
            if dist[idx] < (num, dst) {
                continue;
            }
            for &(nxt, cost) in v[idx].iter() {
                let (num, dst) = if dst + cost <= l {
                    (num, dst + cost)
                } else if cost <= l {
                    (num + 1, cost)
                } else {
                    (inf, inf)
                };

                if dist[nxt] <= (num, dst) {
                    continue;
                }
                dist[nxt] = (num, dst);
                q.push(Reverse((num, dst, nxt)));
            }
        };

        for i in 0..n {
            let dst = if dist[i].0 == inf {
                -1
            } else {
                dist[i].0
            };
            ans[s][i] = dst;
        }
    }

    for (s, t) in query {
        println!("{}", ans[s][t]);
    }
}