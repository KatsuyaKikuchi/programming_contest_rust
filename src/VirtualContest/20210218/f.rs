use proconio::input;

fn main()
{
    input! {
    (x,y,r):(f64,f64,f64)
    }
    let mul = 10_000;
    let (x, y, r) =
        ((x * 10_000.0).round() as i64,
         (y * 10_000.0).round() as i64,
         (r * 10_000.0).round() as i64);
    let (top, bottom) = ((y + r) / mul + 1, (y - r) / mul - 1);
    let mut ans = 0;
    for dy in bottom..=top {
        let (rr, yy) = (r * r, (y - dy * mul).pow(2));
        if rr < yy {
            continue;
        }
        let dx = ((rr - yy) as f64).sqrt() as i64;

        let mut left = (x - dx) / mul + 1;
        while (x - (left - 1) * mul).pow(2) + (y - dy * mul).pow(2) <= r * r {
            left -= 1;
        }
        let mut right = (x + dx) / mul - 1;
        while (x - (right + 1) * mul).pow(2) + (y - dy * mul).pow(2) <= r * r {
            right += 1;
        }
        //println!("{} {}", right, left);
        if left > right {
            continue;
        }
        ans += right - left + 1;
    }
    println!("{}", ans);
}