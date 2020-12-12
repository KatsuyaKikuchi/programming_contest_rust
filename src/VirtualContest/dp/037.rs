use proconio::input;
use std::cmp::max;

fn main()
{
    input! {
    n:usize,
    mut k:i64,
    mut v:[i64;n]
    }
    if k == 0 {
        let ans = v.iter().sum::<i64>();
        println!("{}", ans);
        return;
    }
    let mut s = Vec::new();
    while k > 0 {
        s.push((k % 2) as usize);
        k /= 2;
    }
    s.reverse();

    let mut sum = 0;
    for i in 0..n {
        let bit = (1i64 << s.len()) - 1;
        sum += v[i] & (!bit);
        v[i] &= bit;
    }

    let mut dp = vec![vec![vec![0; 2]; 2]; s.len()];
    let mut num = 0;
    let bit = 1i64 << (s.len() - 1);
    for i in 0..n {
        num += if (v[i] & bit) > 0 { 1 } else { 0 };
    }
    dp[0][0][0] = num * bit;
    dp[0][1][1] = (n as i64 - num) * bit;

    for i in 1..s.len() {
        let bit = 1i64 << (s.len() - i - 1);
        let mut num = 0;
        for j in 0..n {
            num += if (v[j] & bit) > 0 { 1 } else { 0 };
        }
        let (zero, one) = (num * bit, (n as i64 - num) * bit);
        dp[i][0][0] = max(dp[i - 1][0][0], dp[i - 1][1][0]) + zero;
        dp[i][1][0] = max(dp[i - 1][0][0], dp[i - 1][1][0]) + one;

        if s[i] == 1 {
            dp[i][0][0] = max(dp[i - 1][s[i - 1]][1] + zero, dp[i][0][0]);
            dp[i][1][1] = dp[i - 1][s[i - 1]][1] + one;
        } else {
            dp[i][0][1] = dp[i - 1][s[i - 1]][1] + zero;
        }
    }

    let mut ans = 0;
    for i in 0..2 {
        for j in 0..2 {
            ans = max(dp[s.len() - 1][i][j], ans);
        }
    }

    println!("{}", ans + sum);
}