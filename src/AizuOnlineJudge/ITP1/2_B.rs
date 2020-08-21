use proconio::input;

fn main()
{
    input! {
    a:i32,
    b:i32,
    c:i32,
    }

    let ans = if a < b && b < c {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}