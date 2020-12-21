use proconio::input;
use proconio::marker::Chars;
use std::cmp::{min};

fn main()
{
    input! {
    s:Chars
    }

    let n = s.len();
    let inf = 1i64 << 60;
    let mut dp = vec![vec![vec![inf; 2]; n + 1]; n + 1];
    dp[1][1][1] = if s[0] == ')' { 1 } else { 0 };
    for i in 1..n {
        for j in 0..n {
            if s[i] == '(' {
                dp[i + 1][j + 1][0] = min(dp[i][j][0], dp[i][j][1]);
                dp[i + 1][j + 1][1] = min(dp[i + 1][j + 1][1], dp[i][j][1] + 1);
                if j > 0 {
                    dp[i + 1][j - 1][1] = min(dp[i + 1][j - 1][1], dp[i][j][1] + 2);
                }
            } else {
                if j > 0 {
                    dp[i + 1][j - 1][0] = min(dp[i][j][0], dp[i][j][1]);
                    dp[i + 1][j - 1][1] = min(dp[i + 1][j - 1][1], dp[i][j][1] + 1);
                }
                dp[i + 1][j + 1][1] = min(dp[i + 1][j + 1][1], dp[i][j][1] + 2);
            }
        }
    }

    let ans = min(dp[n][0][0], dp[n][0][1]);
    println!("{}", ans);
}