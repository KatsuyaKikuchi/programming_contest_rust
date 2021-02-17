use proconio::input;
use std::cmp::max;

fn main()
{
    input! {
    n:usize,
    c:i64,
    v:[(i64,i64);n]
    }

    let mut left = vec![0; n + 1];
    let mut sum = 0;
    let mut ans = 0;
    for i in 1..(n + 1) {
        sum += v[i - 1].1;
        left[i] = max(left[i - 1], sum - v[i - 1].0);
        ans = max(ans, sum - v[i - 1].0);
    }
    let mut right = vec![0; n + 1];
    let mut sum = 0;
    for i in (0..n).rev() {
        sum += v[i].1;
        right[i] = max(right[i + 1], sum - (c - v[i].0));
        ans = max(ans, sum - (c - v[i].0));
        ans = max(ans, sum - 2 * (c - v[i].0) + left[i]);
    }
    let mut sum = 0;
    for i in 0..n {
        sum += v[i].1;
        ans = max(ans, sum - 2 * v[i].0 + right[i + 1]);
    }
    println!("{}", ans);
}