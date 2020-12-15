use proconio::input;
use proconio::marker::Chars;

fn main()
{
    input! {
    (n,m):(usize,usize),
    v:[Chars;m]
    }
    let v = v.into_iter().map(|s| {
        if s.len() == 8 {
            0
        } else if s[0] == 'F' {
            3
        } else {
            5
        }
    }).collect::<Vec<usize>>();

    let md = 1_000_000_007i64;
    let mut dp = vec![vec![vec![0; 15]; m + 1]; n + 1];
    dp[0][0][0] = 1;
    for x in 1..=9 {
        if x % 3 == 0 {
            if m > 0 && v[0] == 3 {
                dp[1][1][x] = 1;
            }
        } else if x % 5 == 0 {
            if m > 0 && v[0] == 5 {
                dp[1][1][x] = 1;
            }
        } else {
            dp[1][0][x] = 1;
        }
    }
    for i in 1..n {
        for j in 0..=m {
            for k in 0..15 {
                for x in 0..=9 {
                    let nxt = (k * 10 + x) % 15;
                    if nxt == 0 {
                        if j < m && v[j] == 0 {
                            dp[i + 1][j + 1][nxt] += dp[i][j][k];
                            dp[i + 1][j + 1][nxt] %= md;
                        }
                    } else if nxt % 3 == 0 {
                        if j < m && v[j] == 3 {
                            dp[i + 1][j + 1][nxt] += dp[i][j][k];
                            dp[i + 1][j + 1][nxt] %= md;
                        }
                    } else if nxt % 5 == 0 {
                        if j < m && v[j] == 5 {
                            dp[i + 1][j + 1][nxt] += dp[i][j][k];
                            dp[i + 1][j + 1][nxt] %= md;
                        }
                    } else {
                        dp[i + 1][j][nxt] += dp[i][j][k];
                        dp[i + 1][j][nxt] %= md;
                    }
                }
            }
        }
    }

    let ans = dp[n][m].iter().sum::<i64>() % md;
    println!("{}", ans);
}