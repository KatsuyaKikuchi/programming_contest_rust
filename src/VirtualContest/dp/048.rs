use proconio::input;
use std::collections::VecDeque;

fn main()
{
    input! {
    (n,k):(usize,usize),
    v:[usize;n]
    }

    let MOD = 1000000007i64;
    let mut dp = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        let mut q = VecDeque::new();
        let mut sum = 0;
        for j in 0..=k {
            if q.len() > v[i] {
                sum = (MOD + sum - q.pop_front().unwrap()) % MOD;
            }
            sum = (sum + dp[i][j]) % MOD;
            q.push_back(dp[i][j]);
            dp[i + 1][j] = sum;
        }
    }

    let ans = dp[n][k];
    println!("{}", ans);
}