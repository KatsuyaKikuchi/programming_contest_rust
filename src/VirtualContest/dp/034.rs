use proconio::input;
use std::cmp::{Reverse, max, min};

fn main()
{
    input! {
    (m,n):(usize,usize),
    mut p:[i64;m],
    v:[(usize,i64);n]
    }
    p.sort_by_key(|&x| Reverse(x));
    let mut s = vec![0; m + 1];
    for i in 0..p.len() {
        s[i + 1] = s[i] + p[i];
    }

    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..=m {
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            let nxt = min(m, j + v[i].0);
            let cost = s[nxt] - s[j] - v[i].1;
            dp[i + 1][nxt] = max(dp[i + 1][nxt], dp[i][j] + cost);
        }
        //println!("{:?}", dp[i + 1]);
    }

    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}