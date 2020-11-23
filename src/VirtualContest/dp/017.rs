use proconio::input;
use std::cmp::{min, max};

fn main()
{
    input! {
    (x,y):(usize,usize),
    n:usize,
    v:[(usize,i64);n]
    }

    let mut dp = vec![vec![vec![0; y + 1]; x + 1]; n + 1];
    for (i, &(t, h)) in v.iter().enumerate() {
        for j in 0..x + 1 {
            for k in 0..y + 1 {
                dp[i + 1][j][k] = max(dp[i + 1][j][k], dp[i][j][k]);
                let p = min(y - k, t - 1);
                let q = t - p;
                if j + q > x {
                    continue;
                }
                dp[i + 1][j + q][k + p] = max(dp[i + 1][j + q][k + p], dp[i][j][k] + h);
            }
        }
    }
    let mut ans = 0;
    for i in 0..x + 1 {
        for j in 0..y + 1 {
            ans = max(ans, dp[n][i][j]);
        }
    }
    println!("{}", ans);
}