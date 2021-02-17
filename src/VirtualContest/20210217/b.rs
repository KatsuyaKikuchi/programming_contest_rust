use proconio::input;
use proconio::marker::Chars;

fn check(s: Vec<char>) -> bool {
    if s.len() <= 3 {
        return false;
    };
    if s[0] != 'Y' || s[1] != 'A' || s[2] != 'K' || s[3] != 'I' {
        return false;
    }
    true
}

fn main()
{
    input! {
    s:Chars
    }

    let ans = if check(s) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans)
}