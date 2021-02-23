use proconio::input;

fn main()
{
    input! {
    (h,w):(i64,i64)
    }
    let ans = if h == 1 || w == 1 {
        1
    } else {
        (h * w + 1) / 2
    };
    println!("{}", ans);
}