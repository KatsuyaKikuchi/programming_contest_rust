use proconio::input;

fn main()
{
    input! {
    (w,h):(usize,usize)
    }
    let md = 100000;
    let mut dp = vec![vec![vec![0; 2]; w + 1]; h + 1];
    dp[1][0][0] = 1;
    dp[0][1][1] = 1;
    for i in 0..h {
        for j in 0..w {
            dp[i][j][0] %= md;
            dp[i][j][1] %= md;
            dp[i][j + 1][1] += dp[i][j][1];
            dp[i + 1][j][0] += dp[i][j][0];
            if i < h - 1 {
                dp[i + 2][j][0] += dp[i][j][1];
            }
            if j < w - 1 {
                dp[i][j + 2][1] += dp[i][j][0];
            }
        }
    }

    let ans = dp[h][w - 1][0] + dp[h - 1][w][1];
    println!("{}", ans % md);
}