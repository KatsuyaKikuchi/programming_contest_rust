use proconio::input;
use proconio::marker::Chars;
use std::cmp::min;

fn main()
{
    input! {mut s:Chars}

    let inf = 1i64 << 60;
    let mut dp = vec![vec![inf; 2]; s.len() + 1];
    dp[0][0] = 0;
    s.reverse();
    for i in 0..s.len() {
        let n = s[i].to_digit(10).unwrap() as i64;
        if n < 9 {
            dp[i + 1][0] = min(dp[i][1] + n + 1, dp[i][0] + n);
        } else {
            dp[i + 1][0] = dp[i][0] + n;
        }
        dp[i + 1][1] = min(dp[i][0] + 10 - n, dp[i][1] + 9 - n);
        // println!("{} {}", dp[i + 1][0], dp[i + 1][1]);
    }

    let ans = min(dp[s.len()][0], dp[s.len()][1] + 1);
    println!("{}", ans);
}