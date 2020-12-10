use proconio::input;
use proconio::marker::Chars;

fn main()
{
    input! {
        s: Chars,
        d: usize,
    }
    ;
    let md = 1000_000_007;
    let mut dp = vec![vec![vec![0; 2]; d]; s.len()];
    let n = s[0].to_digit(10).unwrap() as usize;
    for i in 0..n {
        dp[0][i % d][0] += 1;
    }
    dp[0][n % d][1] = 1;
    for i in 1..s.len() {
        for m in 0..d {
            for j in 0..=9 {
                dp[i][(j + m) % d][0] += dp[i - 1][m][0];
                dp[i][(j + m) % d][0] %= md;
            }
            let n = s[i].to_digit(10).unwrap() as usize;
            for j in 0..n {
                dp[i][(j + m) % d][0] += dp[i - 1][m][1];
            }
            dp[i][(n + m) % d][1] += dp[i - 1][m][1];
        }
    }

    let ans = (md + dp[s.len() - 1][0].iter().sum::<i64>() - 1) % md;
    println!("{}", ans);
}