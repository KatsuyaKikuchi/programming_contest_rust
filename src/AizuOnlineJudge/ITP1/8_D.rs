use proconio::input;

fn main()
{
    input! {
    s:String,
    p:String,
    }

    let ans = if s.repeat(2).contains(&p) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}