use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main()
{
    input! {
    (n,m):(usize,i64),
    edge:[(Usize1,Usize1);n-1]
    }
    if n == 1 {
        println!("1");
        return;
    }
    let mut v = vec![vec![]; n];
    for (a, b) in edge {
        v[a].push(b);
        v[b].push(a);
    }
    let mut parent = vec![(n, n); n];
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(p) = q.pop_front() {
        for (i, &nxt) in v[p].iter().enumerate() {
            if parent[p].0 == nxt {
                continue;
            }
            parent[nxt] = (p, i);
            q.push_back(nxt);
        }
    }
    let mut dp = vec![1; n];
    let mut count = vec![0; n];
    count[0] = v[0].len();
    let mut q = VecDeque::new();
    for i in 1..n {
        if v[i].len() == 1 {
            q.push_back(i);
        }
        count[i] = v[i].len() - 1;
    }

    while let Some(idx) = q.pop_front() {
        for &child in v[idx].iter() {
            if parent[idx].0 == child {
                continue;
            }
            dp[idx] = (dp[idx] * (dp[child] + 1)) % m;
        }

        let p = parent[idx].0;
        if p < n {
            count[p] -= 1;
            if count[p] == 0 {
                q.push_back(p);
            }
        }
    }

    let mut sums = vec![Vec::new(); n];
    let mut ans = vec![1; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(idx) = q.pop_front() {
        for &child in v[idx].iter() {
            if parent[idx].0 == child {
                ans[idx] *= sums[child][parent[idx].1] + 1;
            } else {
                q.push_back(child);
                ans[idx] *= dp[child] + 1;
            };
            ans[idx] %= m;
        }
        sums[idx] = vec![1; v[idx].len()];
        let mut mul = 1;
        for i in 0..v[idx].len() - 1 {
            let child = v[idx][i];
            if parent[idx].0 == child {
                mul *= sums[child][parent[idx].1] + 1;
            } else {
                mul *= dp[child] + 1;
            };
            mul %= m;
            sums[idx][i + 1] *= mul;
        }
        let mut mul = 1;
        for i in (1..v[idx].len()).rev() {
            let child = v[idx][i];
            if parent[idx].0 == child {
                mul *= sums[child][parent[idx].1] + 1;
            } else {
                mul *= dp[child] + 1;
            };
            mul %= m;
            sums[idx][i - 1] = (sums[idx][i - 1] * mul) % m;
        }
    }

    for value in ans {
        println!("{}", value);
    }
}