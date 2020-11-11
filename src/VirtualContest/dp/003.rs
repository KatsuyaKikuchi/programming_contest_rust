use proconio::input;
use std::collections::VecDeque;

fn main()
{
    input! {
    n:usize,
    (ng0,ng1,ng2):(usize,usize,usize)
    }

    let inf = 1i64 << 60;
    let mut dist = vec![inf; n + 1];
    dist[n] = 0;

    let mut q = VecDeque::new();
    q.push_back(n);
    while let Some(idx) = q.pop_front() {
        if ng0 == idx || ng1 == idx || ng2 == idx {
            continue;
        }
        let dst = dist[idx] + 1;
        for t in 1..=3 {
            if idx < t {
                continue;
            }
            let nxt = idx - t;
            if nxt == ng0 || nxt == ng2 || nxt == ng1 {
                continue;
            }
            if dist[nxt] <= dst {
                continue;
            }
            dist[nxt] = dst;
            q.push_back(nxt);
        }
    }

    let ans = if dist[0] <= 100 {
        "YES"
    } else {
        "NO"
    };
    println!("{}", ans);
}