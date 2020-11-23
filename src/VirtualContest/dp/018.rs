use proconio::input;
use std::cmp::{max, min};

fn main()
{
    input! {
    (n,p):(usize,usize),
    mut v:[(usize,i64);n],
    }
    v.sort_by_key(|&(a, _)| a);

    let mut dp = vec![vec![0; p + 2]; n + 1];
    for (i, &(a, b)) in v.iter().rev().enumerate() {
        for j in 0..p + 2 {
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            if j <= p {
                let x = min(p + 1, j + a);
                dp[i + 1][x] = max(dp[i + 1][x], dp[i][j] + b);
            }
        }
    }
    let ans = dp[n].iter().max().unwrap().clone();
    println!("{}", ans);
}