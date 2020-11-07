use proconio::input;
use std::cmp::max;

fn main()
{
    input! {
    n:usize,
    m:i64,
    p:[i64;n],
    }

    let mut v = Vec::new();
    for i in 0..n {
        for j in 0..n {
            v.push(p[i] + p[j]);
        }
        v.push(p[i]);
    }
    v.push(0);
    v.sort();
    v.dedup();

    let mut ans = 0;
    for i in 0..v.len() {
        if v[i] + v[0] > m {
            break;
        }
        let (mut ok, mut ng) = (0, v.len());
        while (ng - ok) > 1 {
            let mid = (ok + ng) / 2;
            if v[mid] + v[i] <= m {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ans = max(ans, v[i] + v[ok]);
    }

    println!("{}", ans);
}