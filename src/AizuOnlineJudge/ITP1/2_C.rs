use proconio::input;

fn main()
{
    input! {
    mut v:[i32;3],
    }

    v.sort();
    for i in v {
        print!("{} ", i);
    }
    println!("");
}