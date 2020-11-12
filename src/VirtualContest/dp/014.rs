use proconio::input;
use proconio::marker::Chars;
use std::cmp::min;

fn main()
{
    input! {
    size:usize,
    s:Chars
    }
    let mut ans = size;
    for i in 1..size {
        //[0,i)と[i,n)の最長共通部分列
        let (n, m) = (i, size - i);
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for a in 0..n {
            for b in 0..m {
                if s[a] == s[b + i] {
                    dp[a + 1][b + 1] = dp[a][b] + 1;
                }
                dp[a + 1][b + 1] = vec![dp[a + 1][b + 1], dp[a][b + 1], dp[a + 1][b]].iter().max().unwrap().clone();
            }
        }
        ans = min(ans, size - 2 * dp[n][m]);
    }
    println!("{}", ans);
}