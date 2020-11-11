use proconio::input;
use proconio::marker::Usize1;

fn main()
{
    input! {
    (n,k):(usize,usize),
    v:[(Usize1,usize);k]
    }
    let mut dp = vec![vec![vec![0; 4]; 4]; n + 1];
    let mut used = vec![0; n + 1];
    for (a, b) in v {
        used[a] = b;
    }
    for i in 1..4 {
        if used[0] > 0 && used[0] != i {
            continue;
        }
        dp[1][i][0] = 1;
    }

    let md = 10000;
    for i in 1..n {
        for j in 1..4 {
            if used[i] > 0 && used[i] != j {
                continue;
            }
            for p0 in 1..4 {
                for p1 in 0..4 {
                    if p0 == p1 && p1 == j {
                        continue;
                    }
                    dp[i + 1][j][p0] += dp[i][p0][p1];
                    dp[i + 1][j][p0] %= md;
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..4 {
        for j in 0..4 {
            ans += dp[n][i][j];
        }
    }
    println!("{}", ans % md);
}