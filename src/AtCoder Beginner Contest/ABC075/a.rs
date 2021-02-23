use proconio::input;

fn main()
{
    input! {
    (a,b,c):(i32,i32,i32)
    }
    let ans = if a == b {
        c
    } else if a == c {
        b
    } else {
        a
    };
    println!("{}", ans);
}