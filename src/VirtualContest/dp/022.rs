use proconio::input;
use proconio::marker::Usize1;
use std::cmp::min;

fn main()
{
    input! {
    (n,m):(usize,usize)
    }
    let p = 2usize.pow(n as u32);
    let inf = 1i64 << 60;
    let mut dp = vec![inf; p];

    let mut v = Vec::new();
    for _ in 0..m {
        input! {
        (a,b):(i64,usize),
        c:[Usize1;b],
        }
        let mut bit = 0;
        for i in 0..b {
            bit |= 1usize << c[i];
        }
        v.push((a, bit));
    }

    dp[0] = 0;
    for i in 0..p {
        for &(a, b) in v.iter() {
            dp[i | b] = min(dp[i | b], dp[i] + a);
        }
    }

    let ans = if dp[p - 1] == inf {
        -1
    } else {
        dp[p - 1]
    };
    println!("{}", ans);
}