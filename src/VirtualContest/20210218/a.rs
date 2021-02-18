use proconio::input;

fn main()
{
    input! {(x,a,b):(i64,i64,i64)}
    println!("{}", (x - a) % b);
}