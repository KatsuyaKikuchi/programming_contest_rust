use proconio::input;

fn main()
{
    input! {
    (p,q,r):(i64,i64,i64)
    }
    let arr = [p, q, r].iter().max().unwrap().clone();
    println!("{}", p + q + r - arr);
}