use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main()
{
    input! {
    _n:usize,
    k:Usize1,
    mut s:Chars,
    }

    let ans = s.into_iter().enumerate().map(|(i, c)| {
        if i == k {
            c.to_ascii_lowercase()
        } else {
            c
        }
    }).collect::<String>();
    println!("{}", ans);
}