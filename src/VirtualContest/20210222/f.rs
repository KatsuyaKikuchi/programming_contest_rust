use proconio::input;
use proconio::marker::Usize1;
use std::cmp::min;
use std::collections::VecDeque;

fn main()
{
    input! {
    (n,m):(usize,usize),
    edge:[(Usize1,Usize1);m],
    k:usize,
    c:[Usize1;k]
    }

    let mut vertex = vec![Vec::new(); n];
    for (a, b) in edge {
        vertex[a].push(b);
        vertex[b].push(a);
    }

    let inf = 1_000_000_000_000_000i64;
    let mut dist = vec![vec![inf; n]; k];
    for i in 0..k {
        let mut q = VecDeque::new();
        q.push_back(c[i]);
        dist[i][c[i]] = 0;
        while let Some(idx) = q.pop_front() {
            let cost = dist[i][idx] + 1;
            for &nxt in vertex[idx].iter() {
                if dist[i][nxt] <= cost {
                    continue;
                }
                dist[i][nxt] = cost;
                q.push_back(nxt);
            }
        }
    }

    let p = (1 << k) as usize;
    let mut dp = vec![vec![inf; k]; p];
    for i in 0..k {
        let b = (1 << i) as usize;
        dp[b][i] = 0;
    }

    for bit in 0..p {
        for i in 0..k {
            if ((bit >> i) & 1) == 0 {
                continue;
            }
            for j in 0..k {
                let mask = (1 << j) as usize;
                if (bit & mask) == 0 || i == j {
                    continue;
                }
                dp[bit][j] = min(dp[bit][j], dp[bit & (!mask)][i] + dist[i][c[j]]);
            }
        }
    }

    let mut ans = inf;
    for i in 0..k {
        ans = min(ans, dp[p - 1][i] + 1);
    }
    if ans == inf {
        ans = -1;
    }
    println!("{}", ans);
}