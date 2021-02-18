use proconio::input;
use std::cmp::min;

fn main()
{
    input! {(a,b,n):(i64,i64,i64)}
    let m = min(n, b - 1);
    println!("{}", (a * m) / b);
}