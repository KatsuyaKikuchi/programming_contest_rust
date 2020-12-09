use proconio::input;
use proconio::marker::Usize1;
use std::cmp::{Reverse, max};

fn main()
{
    input! {
    (n,k):(usize,usize),
    v:[(i64,Usize1);n]
    }

    let mut b = vec![vec![]; 10];
    for (c, g) in v {
        b[g].push(c);
    }
    for i in 0..b.len() {
        b[i].sort_by_key(|&x| Reverse(x));
    }

    let mut dp = vec![vec![0; k + 1]; 11];
    for i in 0..10 {
        for j in 0..=k {
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            let mut sum = 0;
            for n in 0..b[i].len() {
                if j + n + 1 > k {
                    break;
                }
                sum += b[i][n];
                let s = sum + (n * (n + 1)) as i64;
                dp[i + 1][j + n + 1] = max(dp[i + 1][j + n + 1], dp[i][j] + s);
            }
        }
    }

    let ans = dp[10].iter().max().unwrap().clone();
    println!("{}", ans);
}