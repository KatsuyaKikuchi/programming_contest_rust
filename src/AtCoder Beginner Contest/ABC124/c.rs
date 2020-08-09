use proconio::input;
use proconio::marker::Chars;
use std::cmp::min;

fn main()
{
    input! {
    s:Chars,
    }

    let a = s.iter().enumerate()
        .filter(|(i, &c)| (c as i32) % 2 == (*i as i32) % 2)
        .count();
    let b = s.iter().enumerate()
        .filter(|(i, &c)| (c as i32) % 2 != (*i as i32) % 2)
        .count();
    println!("{}", min(a, b));
}