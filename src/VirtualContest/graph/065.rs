use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main()
{
    input! {
    (n,m):(usize,usize),
    edge:[(Usize1,Usize1);m]
    }

    let mut to = vec![vec![]; n];
    let mut from = vec![vec![]; n];
    for (a, b) in edge {
        to[a].push(b);
        from[b].push(a);
    }

    let inf = 1i64 << 60;
    let mut s = n;
    let mut len = inf;
    for i in 0..n {
        let mut dist = vec![inf; n];
        dist[i] = 0;
        let mut q = VecDeque::new();
        q.push_back(i);
        while let Some(idx) = q.pop_front() {
            let cost = dist[idx] + 1;
            for &nxt in to[idx].iter() {
                if nxt == i && cost < len {
                    len = cost;
                    s = i;
                }
                if dist[nxt] <= cost {
                    continue;
                }
                dist[nxt] = cost;
                q.push_back(nxt);
            }
        }
    }

    if s == n {
        println!("-1");
        return;
    }

    let mut dist = vec![inf; n];
    let mut q = VecDeque::new();
    q.push_back((s, 0));
    while let Some((idx, cst)) = q.pop_front() {
        let cost = cst + 1;
        for &nxt in to[idx].iter() {
            if dist[nxt] <= cost {
                continue;
            }
            dist[nxt] = cost;
            q.push_back((nxt, cost));
        }
    }

    let mut ans = Vec::new();
    while dist[s] < inf {
        if let Some(&last) = ans.last() {
            if last == s {
                break;
            }
        }
        ans.push(s);
        for &nxt in from[s].iter() {
            if dist[nxt] == dist[s] - 1 {
                s = nxt;
                break;
            }
        }
    }

    println!("{}", ans.len());
    for a in ans {
        println!("{}", a + 1);
    }
}