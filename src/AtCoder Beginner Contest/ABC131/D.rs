use proconio::input;
use std::ops::Add;

fn check_value<T: Ord + Add<Output=T>>(a: T, v: (T, T)) -> Option<T> {
    let value = a + v.0;
    if value > v.1 {
        return None;
    };
    Some(value)
}

fn main()
{
    input! {
    n:usize,
    mut v:[(i64,i64);n],
    }

    v.sort_by_key(|a| a.1);
    let ans = if let Some(_) = v.iter().try_fold(0, |sum, &value| check_value(sum, value)) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}