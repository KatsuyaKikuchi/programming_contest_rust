use proconio::input;

fn main()
{
    input! {
    W:i32,
    H:i32,
    x:i32,
    y:i32,
    r:i32
    }

    let ans = if 0 <= x - r && x + r <= W && 0 <= y - r && y + r <= H {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}