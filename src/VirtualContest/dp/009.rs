use proconio::input;
use proconio::marker::Chars;
use std::cmp::min;

fn main()
{
    input! {
    n:usize,
    c:Chars
    }
    let cm = (0..16).into_iter().map(|x| (x % 4, x / 4)).collect::<Vec<(i32, i32)>>();
    let v = c.iter().map(|t| {
        match t {
            'A' => 0,
            'B' => 1,
            'X' => 2,
            'Y' => 3,
            _ => -1
        }
    }).collect::<Vec<i32>>();

    let inf = 1i64 << 60;
    let mut ans = inf;

    for &(a, b) in cm.iter() {
        for &(x, y) in cm.iter() {
            let mut dp = vec![inf; n + 1];
            dp[0] = 0;
            dp[1] = 1;
            for i in 1..n {
                dp[i + 1] = dp[i] + 1;
                if (a == v[i - 1] && b == v[i]) || (x == v[i - 1] && y == v[i]) {
                    dp[i + 1] = min(dp[i + 1], dp[i - 1] + 1);
                }
            }
            ans = min(ans, dp[n]);
        }
    }
    println!("{}", ans);
}