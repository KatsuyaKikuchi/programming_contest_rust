use proconio::input;
use std::cmp::max;

fn main() {
    input! {
    n:usize,
    v:[i32;n],
    c:[i32;n],
    }
    let mut ans = 0;
    for i in 0..n {
        ans += max(0, v[i] - c[i]);
    }
    println!("{}", ans);
}