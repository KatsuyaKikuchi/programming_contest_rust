use proconio::input;

fn main()
{
    input! {n:i64}
    println!("{}", 800 * n - 200 * (n / 15));
}