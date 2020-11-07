use proconio::input;
use std::f64::consts::PI;

fn func(t: f64, a: f64, b: f64, c: f64) -> f64 {
    (PI * c * t).sin() * b + a * t
}

fn main()
{
    input! {
    (a,b,c):(f64,f64,f64)
    }

    let (mut left, mut right) = (0.0, 200.0 + b);
    for _ in 0..10000 {
        let mid = (left + right) * 0.5;
        let value = func(mid, a, b, c);
        if value <= 100.0 {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{:.20}", left);
}