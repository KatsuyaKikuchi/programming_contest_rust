use proconio::input;
use proconio::marker::Chars;
use std::cmp::min;

trait Parse {
    fn to_i32(&self) -> i32;
}

impl Parse for char {
    fn to_i32(&self) -> i32 {
        *self as i32 - ('a' as i32)
    }
}

fn main()
{
    input! {
    n:usize,
    c:[(Chars,Chars);n]
    }

    let mut v = vec![vec![]; 26];
    let mut count = vec![0; 26];
    for (a, b) in c {
        let mut push = false;
        for i in 0..min(a.len(), b.len()) {
            if a[i] == b[i] {
                continue;
            }
            let (from, to) = (a[i].to_i32() as usize, b[i].to_i32() as usize);
            v[from].push(to);
            count[to] += 1;
            push = true;
            break;
        }

        if !push && a.len() >= b.len() {
            println!("-1");
            return;
        }
    }

    let mut used = vec![false; 26];
    let mut ans = Vec::new();
    loop {
        let mut update = false;
        for i in 0..26 {
            if used[i] || count[i] > 0 {
                continue;
            }

            used[i] = true;
            for &to in v[i].iter() {
                count[to] -= 1;
            }
            ans.push(i);
            update = true;
            break;
        }
        if !update {
            break;
        }
    }

    if count.iter().filter(|&&n| n > 0).count() > 0 {
        println!("-1");
        return;
    }

    for n in ans {
        let n = n as u8 + 'a' as u8;
        print!("{}", n as char);
    }
    println!("");
}