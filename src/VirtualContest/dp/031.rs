use proconio::input;
use std::cmp::min;

fn main()
{
    input! {
    (n,m):(usize,usize),
    }

    let mut v = vec![vec![]; n];
    for i in 0..n {
        input! {
        k:usize
        }
        for _ in 0..k {
            input! {
           (x,d):(i64,i64)
            }
            v[i].push((x, d));
        }
    }

    let inf = 1i64 << 60;
    let mut dp = vec![vec![vec![inf; m + 2]; 10]; n + 2];
    for i in 0..10 {
        dp[0][i][0] = 0;
        dp[1][i][1] = 0;
    }
    for i in 0..n {
        for j in 0..v[i].len() {
            for k in 0..=m {
                for t in 1..=2 {
                    if i >= n - t {
                        dp[i + t][0][k + t - 1] = min(dp[i + t][0][k + t - 1], dp[i][j][k]);
                        continue;
                    }
                    for nxt in 0..v[i + t].len() {
                        let cost = (v[i][j].0 - v[i + t][nxt].0).abs() * (v[i][j].1 + v[i + t][nxt].1);
                        dp[i + t][nxt][k + t - 1] = min(dp[i + t][nxt][k + t - 1], dp[i][j][k] + cost);
                    }
                }
            }
        }
    }

    let mut ans = inf;
    for i in 0..=m {
        ans = min(ans, dp[n][0][i]);
    }
    println!("{}", ans);
}