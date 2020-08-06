use proconio::input;
use std::mem::swap;
use std::cmp::max;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a < b {
        let t = a;
        a = b;
        b = t;
    }
    let c = a % b;
    if c == 0 {
        b
    } else {
        gcd(b, c)
    }
}

fn main() {
    input! {
    n:usize,
    a:[i32;n],
    }

    let mut ans = 1;
    let mut v = vec![1; n];
    v[n - 1] = a[n - 1];
    for i in (0..n - 1).rev() {
        v[i] = gcd(v[i + 1], a[i]);
    }

    let mut m = a[0];
    for i in 1..n - 1 {
        ans = max(ans, gcd(m, v[i + 1]));
        m = gcd(m, a[i]);
    }
    ans = max(ans, max(m, v[1]));

    println!("{}", ans);
}