use proconio::input;
use std::cmp::max;

fn main()
{
    input! {
    (n,t,s):(usize,usize,usize),
    v:[(i64,usize);n],
    }

    let mut dp = vec![vec![0; t + 1]; n + 1];
    for (i, &(a, b)) in v.iter().enumerate() {
        for j in 0..t + 1 {
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            if j + b > t {
                continue;
            }
            if j >= s || j + b <= s {
                dp[i + 1][j + b] = max(dp[i + 1][j + b], dp[i][j] + a);
            }
        }
    }

    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}