use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;
use std::collections::VecDeque;

fn main()
{
    input! {
    n:i32,
    mut k:i32,
    s:Chars
    }

    let mut ans = 0;
    let mut q: VecDeque<char> = VecDeque::new();
    let mut index = 0;
    while index < n {
        let c = s[index as usize];
        while index < n && c == s[index as usize] {
            q.push_back(c);
            index += 1;
        }
        if c == '0' {
            k -= 1;
        }
        if k < 0 {
            while *q.front().unwrap() == '1' {
                q.pop_front();
            }
            while *q.front().unwrap() == '0' {
                q.pop_front();
            }
            k += 1;
        }
        ans = max(ans, q.len() as i32);
    }

    println!("{}", ans);
}