use proconio::input;

fn main()
{
    input! {
    t:i32,
    }

    let (h, m, s) = (t / (60 * 60), (t / 60) % 60, t % 60);
    println!("{}:{}:{}", h, m, s);
}