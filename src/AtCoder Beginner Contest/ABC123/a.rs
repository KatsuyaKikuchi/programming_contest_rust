use proconio::input;

fn main()
{
    input! {
    (a,b,c,d,e,k):(i32,i32,i32,i32,i32,i32),
    }

    let ans = if e - a <= k {
        "Yay!"
    } else {
        ":("
    };
    println!("{}", ans);
}