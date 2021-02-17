use proconio::input;
use proconio::marker::Chars;
use std::cmp::min;

fn main()
{
    input! {
    n:usize,
    s:Chars,
    }

    let mut left = vec![0; n + 1];
    for i in 0..n {
        if s[i] == 'W' {
            left[i + 1] += 1;
        }
        left[i + 1] += left[i];
    }
    let mut ans = 1000000000;
    let mut right = 0;
    for i in (0..n).rev() {
        ans = min(ans, right + left[i]);
        if s[i] == 'E' {
            right += 1;
        }
    }
    println!("{}", ans);
}