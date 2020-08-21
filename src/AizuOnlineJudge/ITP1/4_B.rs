use proconio::input;
use std::f64::consts::PI;

fn main()
{
    input! {
    r:f64
    }

    println!("{:.9} {:.9}", r * r * PI, 2.0 * r * PI);
}