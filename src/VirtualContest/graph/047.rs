use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;
use std::cmp::max;


fn main()
{
    input! {
    (n,m):(usize,usize),
    p:i64,
    edge:[(Usize1,Usize1,i64);m],
    }

    let mut count = vec![0; n];
    let mut v = vec![vec![]; n];
    for &(a, b, _) in edge.iter() {
        v[a].push(b);
    }
    let mut q = VecDeque::new();
    q.push_back(0);
    let mut used = vec![false; n];
    used[0] = true;
    while let Some(idx) = q.pop_front() {
        count[idx] += 1;
        for &nxt in v[idx].iter() {
            if used[nxt] {
                continue;
            }
            used[nxt] = true;
            q.push_back(nxt);
        }
    }
    let mut v = vec![vec![]; n];
    for &(a, b, _) in edge.iter() {
        v[b].push(a);
    }
    let mut q = VecDeque::new();
    q.push_back(n - 1);
    let mut used = vec![false; n];
    used[n - 1] = true;
    while let Some(idx) = q.pop_front() {
        count[idx] += 1;
        for &nxt in v[idx].iter() {
            if used[nxt] {
                continue;
            }
            used[nxt] = true;
            q.push_back(nxt);
        }
    }

    let enable = count.iter().map(|&n| n >= 2).collect::<Vec<bool>>();
    let edge = edge.iter()
        .map(|&(a, b, c)| (a, b, p - c)).collect::<Vec<_>>();

    let inf = 1i64 << 60;
    let mut dist = vec![inf; n];
    dist[0] = 0;

    for _ in 0..(n + 5) {
        let mut update = false;
        for &(a, b, c) in edge.iter() {
            if !enable[a] || !enable[b] {
                continue;
            }

            let cost = c + dist[a];
            if dist[b] <= cost {
                continue;
            }

            update = true;
            dist[b] = cost;
        }

        if update {
            continue;
        }

        let ans = max(-dist[n - 1], 0);
        println!("{}", ans);
        return;
    }

    println!("-1");
}