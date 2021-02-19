use proconio::input;
use proconio::marker::Chars;

fn main()
{
    input! {
    s:Chars
    }
    let mut i: usize = 0;
    let mut ans = 0;
    let mut count = 0i64;
    while i < s.len() {
        if s[i] == 'A' {
            count += 1;
        } else if i + 1 >= s.len() || s[i] == 'C' {
            count = 0;
        } else if s[i + 1] == 'A' || s[i + 1] == 'B' {
            count = 0;
        } else {
            ans += count;
            i += 1;
        }
        i += 1;
    }
    println!("{}", ans);
}