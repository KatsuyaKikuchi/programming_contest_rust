use proconio::input;

fn main()
{
    input! {
    (a,b,c,d) :(usize,usize,usize,usize)
    }

    let mut dp = vec![vec![0; d]; c];
    dp[a - 1][b - 1] = 1;
    let md = 998244353i64;
    for i in a..c {
        dp[i][b - 1] = (dp[i - 1][b - 1] * (b as i64)) % md;
    }
    for i in b..d {
        dp[a - 1][i] = (dp[a - 1][i - 1] * (a as i64)) % md;
    }

    for i in a..c {
        for j in b..d {
            let (x, y, z) = (
                (dp[i - 1][j] * ((j + 1) as i64)) % md,
                (dp[i][j - 1] * ((i + 1) as i64)) % md,
                (dp[i - 1][j - 1] * (i as i64) * (j as i64)) % md
            );
            dp[i][j] = (x + y - z + md) % md;
        }
    }

    println!("{}", dp[c - 1][d - 1]);
}