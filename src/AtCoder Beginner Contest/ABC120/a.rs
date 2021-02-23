use proconio::input;
use std::cmp::min;

fn main()
{
    input! {
    (a,b,c):(i32,i32,i32)
    }
    println!("{}", min(c, b / a));
}