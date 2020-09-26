use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;
use std::cmp::min;

fn main()
{
    input! {
    n:usize,
    m:usize,
    s:Usize1,
    edge:[(Usize1,Usize1);m],
    }

    let mut v = vec![vec![]; n];
    for (a, b) in edge {
        v[a].push(b);
        v[b].push(a);
    }
    let mut p = vec![0; n];
    let mut q = BinaryHeap::new();
    q.push((s, s));
    while let Some((mx, idx)) = q.pop() {
        if p[idx] >= mx {
            continue;
        }
        p[idx] = mx;
        let mx = min(idx, mx);
        for &nxt in v[idx].iter() {
            if p[nxt] >= mx {
                continue;
            }

            q.push((mx, nxt));
        }
    }


    for (i, t) in p.into_iter().enumerate() {
        if i <= t {
            println!("{}", i + 1);
        }
    }
}