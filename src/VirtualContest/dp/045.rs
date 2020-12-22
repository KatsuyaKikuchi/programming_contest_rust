use proconio::input;
use std::cmp::min;

fn main()
{
    input! {
    n:usize,
    v:[i64;n]
    }
    let inf = 1i64 << 60;
    let mut dp = vec![vec![inf; n + 1]; n];
    for i in 0..n {
        dp[i][i + 1] = 0;
    }
    for len in 2..=n {
        for i in 0..n {
            if i + len > n {
                break;
            }
            let mut sum = 0;
            for j in 0..len {
                sum += v[i + j];
            }
            for j in 1..len {
                dp[i][i + len] = min(dp[i][i + len], sum + dp[i][i + j] + dp[i + j][i + len]);
            }
        }
    }
    println!("{}", dp[0][n]);
}