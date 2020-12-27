use proconio::input;
use std::cmp::{min, max};

fn main() {
    input! {
    n:usize,
    edge:[(i64,i64);n-1],
    }

    let mut ans = 0;
    for i in 0..n {
        let len = i as i64 + 1;
        let num = (n - i) as i64;
        ans += len * num;
    }

    for (a, b) in edge {
        let (a, b) = (min(a, b), max(a, b));
        let b = n as i64 + 1 - b;
        ans -= a * b;
    }
    println!("{}", ans);
}