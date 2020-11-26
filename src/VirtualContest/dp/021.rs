use proconio::input;
use std::cmp::min;

fn main()
{
    input! {
    n:usize,
    mut d:u64
    }
    let (mut a, mut b, mut c) = (0, 0, 0);
    while d % 2 == 0 {
        d /= 2;
        a += 1;
    }
    while d % 3 == 0 {
        d /= 3;
        b += 1;
    }
    while d % 5 == 0 {
        d /= 5;
        c += 1;
    }
    if (d > 1)
    {
        println!("0");
        return;
    }

    let mut dp = vec![vec![vec![vec![0.0; c + 1]; b + 1]; a + 1]; n + 1];
    dp[0][0][0][0] = 1.0;
    for i in 0..n {
        for a0 in 0..a + 1 {
            for b0 in 0..b + 1 {
                for c0 in 0..c + 1 {
                    let d = dp[i][a0][b0][c0] / 6.0;
                    let (a1, b1, c1, a2) = (min(a, a0 + 1), min(b, b0 + 1),
                                            min(c, c0 + 1), min(a, a0 + 2));
                    dp[i + 1][a0][b0][c0] += d;
                    dp[i + 1][a1][b0][c0] += d;
                    dp[i + 1][a0][b1][c0] += d;
                    dp[i + 1][a2][b0][c0] += d;
                    dp[i + 1][a0][b0][c1] += d;
                    dp[i + 1][a1][b1][c0] += d;
                }
            }
        }
    }
    println!("{:.10}", dp[n][a][b][c]);
}