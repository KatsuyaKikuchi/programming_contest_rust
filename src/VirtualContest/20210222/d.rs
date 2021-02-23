use proconio::input;
use proconio::marker::Chars;
use std::cmp::min;

fn main()
{
    input! {
    n:usize,
    s:Chars
    }
    let mut right = vec![0; n + 1];
    for i in (0..n).rev() {
        right[i] = right[i + 1];
        if s[i] == '.' {
            right[i] += 1;
        }
    }
    let mut left = 0;
    let mut ans = right[0];
    for i in 0..n {
        if s[i] == '#' {
            left += 1;
        }
        ans = min(ans, left + right[i + 1]);
    }
    println!("{}", ans);
}