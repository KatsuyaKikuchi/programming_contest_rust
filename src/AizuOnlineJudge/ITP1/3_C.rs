use proconio::input;
use std::cmp::min;
use std::cmp::max;

fn main()
{
    loop {
        input! {x:i32,y:i32}

        if x == 0 && y == 0 {
            break;
        }

        println!("{} {}", min(x, y), max(x, y));
    }
}