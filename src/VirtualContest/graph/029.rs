use proconio::input;
use proconio::marker::{Usize1, Chars};

fn main()
{
    input! {
    n:usize,
    a:Usize1,
    k:Chars,
    b:[Usize1;n],
    }

    let ans = if k.len() < 6
    {
        let mut p = 0;
        for &c in k.iter() {
            p = p * 10 + ((c as i32) - ('0' as i32));
        }
        let mut nxt = a;
        for _ in 0..p {
            nxt = b[nxt];
        }
        nxt + 1
    } else {
        let inf = 1 << 30;
        let mut v = vec![inf; n];
        v[a] = 0;
        let mut cnt = 1;
        let mut nxt = b[a];
        loop {
            if v[nxt] < cnt {
                break;
            }
            v[nxt] = cnt;
            cnt += 1;
            nxt = b[nxt];
        };
        let (ofs, m) = (v[nxt], cnt - v[nxt]);

        let mut p = 0;
        for &c in k.iter() {
            p = (p * 10 + (c as i32) - ('0' as i32)) % m;
        }
        p = (m + p - (ofs % m)) % m;
        for _ in 0..p {
            nxt = b[nxt];
        }
        nxt + 1
    };

    println!("{}", ans);
}