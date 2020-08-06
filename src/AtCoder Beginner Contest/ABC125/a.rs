use proconio::input;

fn main() {
    input! {
    a:i32,
    b:i32,
    t:i32
    }

    let mut ans = 0;
    for i in 1..=t {
        if i % a == 0 {
            ans += b;
        }
    }
    println!("{}", ans);
}