use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main()
{
    input! {
    n:usize,
    s:Chars
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];
    for j in (1..n).rev() {
        for i in (0..(j - 1)).rev() {
            if s[i] == s[j] {
                let len = dp[i + 1][j + 1] + 1;
                if i + len <= j {
                    dp[i][j] = max(dp[i][j], len);
                }
            }
        }
    }
    let ans = dp.iter().map(|x| x.iter().max().unwrap()).max().unwrap().clone();
    println!("{}", ans);
}