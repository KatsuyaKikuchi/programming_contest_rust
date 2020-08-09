use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main()
{
    input! {
    n:i32,
    mut k:i32,
    s:Chars
    }

    let mut ans = 0;
    let mut last = 0;
    let mut index = 0;
    while index < n {
        if s[index as usize] == '0'
        {
            k -= 1;
            while index + 1 < n && s[index as usize] == '0' {
                index += 1;
            }

            if k < 0 {
                loop {
                    last += 1;
                    if s[last as usize] == '1' && s[(last - 1) as usize] == '0' {
                        break;
                    }
                }
                k += 1;
            }
        }
        ans = max(ans, index - last + 1);
        index += 1;
    }

    println!("{}", ans);
}