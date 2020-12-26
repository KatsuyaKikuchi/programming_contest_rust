use proconio::input;
use std::cmp::max;

fn main()
{
    input! {
    n:usize,
    v:[i64;n]
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];
    if n % 2 == 1 {
        for i in 0..n {
            dp[i][0] = v[i];
        }
    }

    for len in 1..n {
        for i in 0..n {
            let last = (i + len) % n;
            if len % 2 == n % 2 {
                // 妹
                dp[i][len] = if v[i] < v[last] { dp[i][len - 1] } else { dp[(i + 1) % n][len - 1] }
            } else {
                // 兄
                dp[i][len] = max(dp[i][len - 1] + v[last], dp[(i + 1) % n][len - 1] + v[i]);
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        ans = max(dp[i][n - 1], ans);
    }
    println!("{}", ans)
}