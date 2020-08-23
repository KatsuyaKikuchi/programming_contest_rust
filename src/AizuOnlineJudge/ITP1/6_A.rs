use proconio::input;

fn main()
{
    input! {
    n:i32,
    mut v:[i32;n]
    }

    v.reverse();
    for i in v.iter() {
        print!("{} ", i);
    }
    println!("");
}