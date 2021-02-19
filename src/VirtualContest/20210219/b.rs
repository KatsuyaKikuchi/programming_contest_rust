use proconio::input;
use proconio::marker::Chars;

fn main()
{
    input! {
    s:Chars,
    t:Chars
    }

    for i in 0..3 {
        if s[i] != t[2 - i] {
            println!("NO");
            return;
        }
    }
    println!("YES");
}