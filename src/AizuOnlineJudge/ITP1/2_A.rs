use proconio::input;

fn main()
{
    input! {
    a:i32,
    b:i32,
    }

    let ans = if a == b {
        "a == b"
    } else if a > b {
        "a > b"
    } else {
        "a < b"
    };
    println!("{}", ans);
}