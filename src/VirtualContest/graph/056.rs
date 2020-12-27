use proconio::input;
use proconio::marker::Usize1;
use std::cmp::{min};
use std::collections::VecDeque;

fn main()
{
    input! {
    (n,m):(usize,usize),
    v:[i64;n],
    edge:[(Usize1,Usize1);m]
    }

    let mut vtx = vec![Vec::new(); n];
    for (idx, &(a, b)) in edge.iter().enumerate() {
        vtx[a].push((b, idx));
        vtx[b].push((a, idx));
    }

    let mut ans = 0;
    let mut seen = vec![false; n];

    for i in 0..n {
        if seen[i] {
            continue;
        }
        let mut sum = v[i];
        let mut mn = v[i];
        let mut enb = false;
        seen[i] = true;

        let mut q = VecDeque::new();
        q.push_back((i, m));
        while let Some((idx, ei)) = q.pop_front() {
            for &(nxt, nxt_e) in vtx[idx].iter() {
                if seen[nxt] {
                    if ei != nxt_e {
                        enb = true;
                    }
                    continue;
                }
                sum += v[nxt];
                mn = min(mn, v[nxt]);
                q.push_back((nxt, nxt_e));
                seen[nxt] = true;
            }
        }

        ans += if enb { sum } else { sum - mn };
    }

    println!("{}", ans);
}