use proconio::input;
use std::cmp::max;

fn main()
{
    input! {
    n:i32,
    h:[i32;n],
    }

    let mut ans = 0;
    let mut mx = 0;

    for &i in h.iter() {
        mx = max(mx, i);
        if mx <= i { ans += 1; }
    }

    println!("{}", ans);
}