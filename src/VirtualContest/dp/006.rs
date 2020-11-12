use proconio::input;
use std::cmp::min;

fn main()
{
    input! {
    (h,n):(usize,usize),
    v :[(usize,i64);n]
    }

    let inf = 1i64 << 60;
    let mut dp = vec![vec![inf; h + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..(h + 1) {
            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]);
            let nxt = min(j + v[i].0, h);
            dp[i + 1][nxt] = min(dp[i + 1][nxt], dp[i + 1][j] + v[i].1);
        }
    }
    println!("{}", dp[n][h]);
}