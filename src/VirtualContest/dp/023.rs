use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main()
{
    input! {
    (n,m):(usize,usize),
    edge:[(Usize1,Usize1);m]
    }
    let p = 1usize << n;
    let mut dp = vec![0i64; p];
    dp[0] = 1;
    let mut v = vec![vec![]; n];
    for (a, b) in edge {
        v[b].push(a);
    }

    for bit in 0..p {
        let mut q = VecDeque::new();
        let mut used = vec![false; n];
        for i in 0..n {
            if (bit >> i) & 1 == 1 {
                q.push_back(i);
                used[i] = true;
            }
        }

        let mut enable = true;
        while let Some(idx) = q.pop_front() {
            for &nxt in v[idx].iter() {
                if used[nxt] {
                    continue;
                }
                if ((bit >> nxt) & 1) == 0 {
                    enable = false;
                    break;
                }
                q.push_back(nxt);
            }
        }
        if !enable {
            continue;
        }
        for i in 0..n {
            if ((bit >> i) & 1) == 0 {
                continue;
            }
            dp[bit] += dp[bit & (!(1usize << i))];
        }
    }
    println!("{}", dp[p - 1]);
}