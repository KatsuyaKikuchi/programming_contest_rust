use proconio::input;

fn main()
{
    input! {
    r:i32,
    d:i32,
    mut x:i32,
    }

    for i in 0..10 {
        x = r * x - d;
        println!("{}", x);
    }
}