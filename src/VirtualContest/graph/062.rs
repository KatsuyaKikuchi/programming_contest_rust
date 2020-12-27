use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;
use std::cmp::{Reverse, min};

fn main()
{
    input! {
    (n,m,k):(usize,usize,usize),
    tp:[i32;n],
    edge:[(Usize1,Usize1,usize);m]
    }

    let mut v = vec![vec![]; n];
    for (a, b, c) in edge {
        v[a].push((b, c));
        v[b].push((a, c));
    }

    let inf = 1i64 << 60;
    let mut dist = vec![vec![inf; k + 1]; n];
    dist[0][0] = 0;

    let mut q = BinaryHeap::new();
    q.push(Reverse((0, (0, k), 0)));

    while let Some(Reverse((dst, (h, c), idx))) = q.pop() {
        if dist[idx][min(h, c)] < dst {
            continue;
        }

        for &(nxt, cst) in v[idx].iter() {
            let dst = dst + (cst as i64);
            let (mut h, mut c) = (min(h + cst, k), min(c + cst, k));
            if (tp[nxt] == 0 && c < k) || (tp[nxt] == 2 && h < k) {
                continue;
            }
            if tp[nxt] == 0 {
                h = 0;
            }
            if tp[nxt] == 2 {
                c = 0;
            }
            if dist[nxt][min(h, c)] <= dst {
                continue;
            }
            dist[nxt][min(h, c)] = dst;
            q.push(Reverse((dst, (h, c), nxt)));
        }
    }

    let ans = dist[n - 1].iter().min().unwrap();
    println!("{}", ans);
}