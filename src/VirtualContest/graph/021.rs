use proconio::input;
use std::cmp::min;

fn solve(mut a: i64, mut b: i64, n: i64) -> i64 {
    if n == 1 {
        return min(a, b);
    }
    while a != b {
        if a > b {
            a = (a + n - 2) / n;
        } else {
            b = (b + n - 2) / n;
        }
    };
    a
}

fn main()
{
    input! {
    n:i64,
    q:usize,
    query:[(i64,i64);q],
    }

    for (a, b) in query {
        println!("{}", solve(a, b, n));
    }
}