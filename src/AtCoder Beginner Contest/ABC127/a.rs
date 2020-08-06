use proconio::input;

fn main() {
    input! {
    a:i32,
    b:i32,
    }

    let ans = match a {
        0..=5 => 0,
        6..=12 => b / 2,
        _ => b,
    };

    println!("{}", ans);
}
