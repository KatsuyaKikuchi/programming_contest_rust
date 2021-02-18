use proconio::input;

fn main()
{
    input! {
    n:usize,
    mut v:[i64;n]
    }
    v.sort();
    println!("{}", v.last().unwrap() - v[0]);
}