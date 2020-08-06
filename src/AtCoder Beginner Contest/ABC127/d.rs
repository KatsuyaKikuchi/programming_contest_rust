use proconio::input;
use std::collections::BinaryHeap;

fn main()
{
    input! {
    n:i32,
    m:i32,
    a:[i64;n],
    b:[(i64,i64);m],
    }

    let mut q = BinaryHeap::new();
    for i in a {
        q.push((i, 1));
    }

    for (x, y) in b {
        q.push((y, x));
    }

    let mut ans: i64 = 0;
    for _ in 0..n {
        let (x, y) = q.pop().unwrap();
        ans += x;
        if y > 1 {
            q.push((x, y - 1));
        }
    }
    println!("{}", ans);
}