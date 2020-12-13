use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn solve(mut s: Vec<char>, b: usize) -> i64 {
    let mut ret = 0;
    for i in 0..s.len() {
        if s[i] == '?' {
            s[i] = if i % 2 == b { '2' } else { '5' }
        }
    }
    //println!("{:?}", s);

    let mut n = if s[0] == '2' { 1 } else { 0 };
    for i in 1..s.len() {
        if s[i] == '2' {
            if s[i - 1] == '5' {
                n += 1;
            } else {
                n = 1;
            }
        } else if s[i] == '5' && s[i - 1] == '2' {
            n += 1;
        } else {
            n = 0;
        }
        ret = max(ret, n);
    }
    ret - (ret % 2)
}

fn main()
{
    input! {
    mut s:Chars
    }

    let t = s.clone();
    let ans = max(solve(s, 0), solve(t, 1));
    println!("{}", ans - (ans % 2));
}