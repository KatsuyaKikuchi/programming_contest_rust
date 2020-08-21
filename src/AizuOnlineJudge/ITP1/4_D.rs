use proconio::input;

fn main()
{
    input! {
    n:usize,
    v:[i64;n]
    }

    let mn = v.iter().min().unwrap();
    let mx = v.iter().max().unwrap();
    let s = v.iter().sum::<i64>();
    println!("{} {} {}", mn, mx, s);
}