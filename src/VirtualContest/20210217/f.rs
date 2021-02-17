use proconio::input;
use std::cmp::max;

fn main()
{
    input! {
    n:usize,
    v:[i64;n]
    }
    let inf = 1_000_000_000_000_000i64;
    let ans = if n % 2 == 0 {
        let mut dp = vec![-inf; n + 1];
        dp[0] = v[0];
        dp[1] = v[1];
        for i in 2..n {
            dp[i] = if i % 2 == 0 {
                v[i] + dp[i - 2]
            } else {
                v[i] + max(dp[i - 2], dp[i - 3])
            };
        }
        max(dp[n - 1], dp[n - 2])
    } else {
        let mut dp = vec![vec![-inf; 2]; n + 1];
        dp[0][0] = v[0];
        dp[1][0] = v[1];
        dp[2][1] = v[2];
        dp[2][0] = v[2] + v[0];
        for i in 3..n {
            dp[i][0] = if i % 2 == 0 {
                v[i] + dp[i - 2][0]
            } else {
                v[i] + max(dp[i - 2][0], dp[i - 3][0])
            };
            let t = if i % 2 == 0 {
                v[i] + dp[i - 2][1]
            } else {
                v[i] + max(dp[i - 2][1], dp[i - 3][1])
            };
            dp[i][1] = max(v[i] + dp[i - 3][0], t);
        }
        [dp[n - 1][1], dp[n - 2][0], dp[n - 3][0]].iter().max().unwrap().clone()
    };
    println!("{}", ans);
}