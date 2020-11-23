use proconio::input;
use std::cmp::{max, min};

fn main()
{
    input! {
    (n,t):(usize,usize),
    mut v:[(usize,i64);n]
    }
    v.sort_by_key(|&(a, _)| a);
    let mut dp = vec![vec![0; t + 1]; n + 1];
    for (i, &(a, b)) in v.iter().enumerate() {
        for j in 0..t + 1 {
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            if j < t {
                let p = min(t, j + a);
                dp[i + 1][p] = max(dp[i + 1][p], dp[i][j] + b);
            }
        }
    }
    let ans = dp[n].iter().max().unwrap().clone();
    println!("{}", ans);
}