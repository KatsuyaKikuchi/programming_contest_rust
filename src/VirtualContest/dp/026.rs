use proconio::input;
use std::cmp::max;

fn main()
{
    input! {
    (n,_z,w):(usize,i64,i64),
    v:[i64;n]
    }

    let ans = if n > 1 {
        max((v[n - 1] - w).abs(), (v[n - 2] - v[n - 1]).abs())
    } else {
        (v[0] - w).abs()
    };

    println!("{}", ans);
}