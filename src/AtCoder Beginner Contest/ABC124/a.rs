use proconio::input;
use std::cmp::max;

fn main()
{
    input! {
    a:i32,
    b:i32
    }

    let ans = if a == b {
        2 * a
    } else {
        let m = max(a, b);
        m + m - 1
    };
    println!("{}", ans);
}