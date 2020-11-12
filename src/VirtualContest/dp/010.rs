use proconio::input;
use std::cmp::max;

fn main()
{
    input! {
    n:usize,
    r:[i64;n],
    }
    let mut dp = vec![vec![0; 2]; n];
    dp[0][0] = 1;
    dp[0][1] = 1;
    let mut ans = 1;
    for i in 1..n {
        for j in 0..i {
            if r[i] < r[j] {
                dp[i][0] = max(dp[i][0], dp[j][1] + 1);
            }
            if r[i] > r[j] {
                dp[i][1] = max(dp[i][1], dp[j][0] + 1);
            }
        }
        ans = max(ans, dp[i].iter().max().unwrap().clone());
    }

    if ans < 3 {
        ans = 0;
    }
    println!("{}", ans);
}