use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main()
{
    input! {
    (n,m):(usize,usize),
    edge:[(Usize1,Usize1);n-1+m]
    }
    let mut v = vec![Vec::new(); n];
    let mut count = vec![0; n];
    for (a, b) in edge {
        v[a].push(b);
        count[b] += 1;
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        if count[i] == 0 {
            q.push_back(i);
        }
    }

    let mut ans = vec![0; n];
    while let Some(idx) = q.pop_front() {
        for &nxt in v[idx].iter() {
            count[nxt] -= 1;
            if count[nxt] == 0 {
                q.push_back(nxt);
                ans[nxt] = idx + 1;
            }
        }
    }

    for r in ans {
        println!("{}", r);
    }
}