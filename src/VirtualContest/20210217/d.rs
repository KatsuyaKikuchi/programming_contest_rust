use proconio::input;
use std::cmp::max;

fn main()
{
    input! {
    (b,mut c):(i64,i64)
    }
    let mut range = Vec::new();
    range.push((b - (c / 2), b));
    c -= 1;
    range.push((-b, -(b - (c / 2))));
    range.push((-b - (c / 2), -b));
    if c >= 1 {
        c -= 1;
        range.push((b, b + (c / 2)));
    }

    range.sort();
    let mut ans = 0;
    let mut right = -2i64.pow(62);
    for (l, r) in range {
        right = max(right, l);
        ans += max(0, r - right + 1);
        right = max(r + 1, right);
    }
    println!("{}", ans);
}