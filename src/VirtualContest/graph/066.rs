use proconio::input;
use proconio::marker::Usize1;
use std::collections::{VecDeque};

fn main()
{
    input! {
    n:usize,
    edge:[(Usize1,Usize1);n-1],
    m:usize,
    route:[(Usize1,Usize1);m]
    }
    let mut v = vec![vec![]; n];
    for (i, &(a, b)) in edge.iter().enumerate() {
        v[a].push((b, i));
        v[b].push((a, i));
    }

    let mut vs = vec![vec![]; m];
    let inf = 1i64 << 60;
    for (i, &(a, b)) in route.iter().enumerate() {
        let mut q = VecDeque::new();
        q.push_back((a, 0));
        let mut dist = vec![inf; n];
        dist[a] = 0;
        while let Some((idx, cst)) = q.pop_front() {
            let cost = cst + 1;
            for &(nxt, _) in v[idx].iter() {
                if dist[nxt] <= cost {
                    continue;
                }
                dist[nxt] = cost;
                q.push_back((nxt, cost));
            }
        }

        let mut now = b;
        while now != a {
            for &(p, idx) in v[now].iter() {
                if dist[p] == dist[now] - 1 {
                    now = p;
                    vs[i].push(idx);
                    break;
                }
            }
        }
    }

    let mut ans = 0;
    let mx = 2_i64.pow(m as u32) as usize;
    for bit in 0..mx {
        let mut idxs = vec![false; n - 1];
        let mut cnt = 0;
        for b in 0..m {
            if ((bit >> b) & 1) == 1 {
                cnt += 1;
                for &idx in vs[b].iter() {
                    idxs[idx] = true;
                }
            }
        }

        let size = (n - 1 - (idxs.iter().filter(|&&x| x).count())) as u32;

        ans += if cnt % 2 == 0 {
            2_i64.pow(size) as i64
        } else {
            -(2_i64.pow(size) as i64)
        };
    }

    println!("{}", ans);
}