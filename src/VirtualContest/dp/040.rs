use proconio::input;
use std::cmp::max;

fn main()
{
    input! {
    (n,m,k):(usize,usize,usize),
    v:[i64;n]
    }
    let mut dp = vec![vec![-1; k + 1]; m + 2];
    dp[0][0] = 0;
    for i in 0..n {
        let mut tmp = vec![vec![-1; k + 1]; m + 2];
        for j in 0..=m {
            for b in 0..k {
                if dp[j][b] < 0 {
                    continue;
                }
                tmp[j][b + 1] = max(tmp[j][b + 1], dp[j][b]);
                tmp[j + 1][0] = max(tmp[j + 1][0], dp[j][b] + v[i]);
            }
        }
        dp = tmp.clone();
    }
    let mut ans = -1;
    for i in 0..k {
        ans = max(ans, dp[m][i]);
    }
    println!("{}", ans);
}