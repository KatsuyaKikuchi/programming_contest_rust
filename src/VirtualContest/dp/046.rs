use proconio::input;
use std::cmp::max;

fn main()
{
    input! {
    n:usize,
    v:[i64;n]
    }

    let mut dp = vec![vec![vec![0; 2]; n + 1]; n + 1];
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        dp[i][i + 1][0] = v[i];
        sum[i + 1] = sum[i] + v[i];
    }
    for len in 1..=n {
        for i in 0..n {
            if i + len > n {
                break;
            }
            dp[i][i + len][len % 2] = max(dp[i + 1][i + len][len % 2] + v[i],
                                          dp[i][i + len - 1][len % 2] + v[i + len - 1]);
            dp[i][i + len][(len + 1) % 2] = sum[len + i] - sum[i] - dp[i][i + len][len % 2];
        }
    }
    let ans = if n % 2 == 0 { dp[0][n][0] - dp[0][n][1] } else { dp[0][n][1] - dp[0][n][0] };
    println!("{}", ans);
}