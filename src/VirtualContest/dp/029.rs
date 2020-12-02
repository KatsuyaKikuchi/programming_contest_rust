use proconio::input;
use std::cmp::{max, min};

fn main()
{
    input! {
    (h,w):(usize,usize),
    a:[[usize;w];h],
    b:[[usize;w];h]
    }

    let mx = 160 * 85 * 2;
    let mid = mx / 2;
    let mut dp = vec![vec![vec![false; mx]; w + 1]; h + 1];
    dp[0][0][mid + a[0][0] - b[0][0]] = true;
    dp[0][0][mid - a[0][0] + b[0][0]] = true;
    for i in 0..h {
        for j in 0..w {
            for m in 0..mx {
                if !dp[i][j][m] {
                    continue;
                }
                if i + 1 < h {
                    dp[i + 1][j][m + a[i + 1][j] - b[i + 1][j]] = true;
                    dp[i + 1][j][m - a[i + 1][j] + b[i + 1][j]] = true;
                }
                if j + 1 < w {
                    dp[i][j + 1][m + a[i][j + 1] - b[i][j + 1]] = true;
                    dp[i][j + 1][m - a[i][j + 1] + b[i][j + 1]] = true;
                }
            }
        }
    }

    let ans = dp[h - 1][w - 1].iter()
        .enumerate()
        .filter(|&(_i, &b)| b)
        .map(|(i, _b)| max(i, mid) - min(i, mid))
        .min().unwrap();
    println!("{}", ans);
}