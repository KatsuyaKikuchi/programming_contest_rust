use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;
use std::cmp::min;

fn main()
{
    input! {
    (n,m):(usize,usize),
    edge:[(Usize1,Usize1);m],
    s:Usize1,
    k:usize,
    mut t:[Usize1;k]
    }

    let mut v = vec![vec![]; n];
    for (a, b) in edge {
        v[a].push(b);
        v[b].push(a);
    }

    let inf = 1i64 << 60;
    let mut dist = vec![vec![inf; n]; k + 1];
    t.push(s);
    let len = k + 1;

    for i in 0..len {
        let mut q = VecDeque::new();
        q.push_back((t[i], 0));
        dist[i][t[i]] = 0;
        while let Some((idx, cost)) = q.pop_front() {
            if dist[i][idx] < cost {
                continue;
            }

            let cost = cost + 1;
            for &nxt in v[idx].iter() {
                if dist[i][nxt] <= cost {
                    continue;
                }
                dist[i][nxt] = cost;
                q.push_back((nxt, cost));
            }
        }
    }

    // 最後がiで終わり、bitで表される街を通った時のコスト最小値
    let mut dp = vec![vec![inf; 1 << len]; len];
    dp[len - 1][1 << (len - 1)] = 0;

    for bit in 1..(1 << len) {
        for i in 0..len {
            if ((bit >> i) & 1) == 0 {
                continue;
            }
            let b = bit & (!(1 << i));
            for j in 0..len {
                if ((b >> j) & 1) == 0 {
                    continue;
                }
                let cost = dp[j][b] + dist[j][t[i]];
                dp[i][bit] = min(dp[i][bit], cost);
            }
        }
    }

    let mut ans = inf;
    for i in 0..len {
        ans = min(dp[i][(1 << len) - 1], ans);
    }
    println!("{}", ans);
}