use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main()
{
    input! {
    s:Chars
    }
    let n = s.len();
    let mut dp = vec![vec![0; 2]; n + 2];
    dp[1][0] = 1;
    dp[2][1] = 1;
    if n > 1 && s[0] != s[1] {
        dp[2][0] = 2;
    }
    for i in 2..n {
        dp[i + 1][0] = dp[i][1] + 1;
        if s[i] != s[i - 1] {
            dp[i + 1][0] = max(dp[i + 1][0], dp[i][0] + 1);
        }
        dp[i + 1][1] = dp[i - 1][0] + 1;
        if i >= 3 && (s[i] != s[i - 2] || s[i - 1] != s[i - 3]) {
            dp[i + 1][1] = max(dp[i + 1][1], dp[i - 1][1] + 1);
        }
    }
    println!("{}", max(dp[n][0], dp[n][1]));
}