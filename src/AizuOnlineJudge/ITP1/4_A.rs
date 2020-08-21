use proconio::input;

fn main()
{
    input! {
    a:i64,
    b:i64
    }

    let d = a / b;
    let r = a % b;
    let f = (a as f64) / (b as f64);

    println!("{} {} {:.9}", d, r, f);
}