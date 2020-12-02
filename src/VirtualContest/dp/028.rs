use proconio::input;

fn main()
{
    input! {
    (n,m):(usize,usize),
    s:[i64;n],
    t:[i64;m]
    }

    let mut dp = vec![vec![1; m + 1]; n + 1];
    let md = 1000_000_007i64;
    for i in 0..n {
        for j in 0..m {
            dp[i + 1][j + 1] = (md + dp[i][j + 1] + dp[i + 1][j] - dp[i][j]) % md;
            if s[i] == t[j] {
                dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j]) % md;
            }
        }
    }

    println!("{}", dp[n][m]);
}