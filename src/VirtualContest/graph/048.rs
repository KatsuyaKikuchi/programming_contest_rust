use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main()
{
    input! {
    (n,m):(usize,usize),
    edge:[(Usize1,Usize1);m],
    }

    let mut v = vec![vec![]; n];
    for (a, b) in edge
    {
        v[a].push(b);
        v[b].push(a);
    }

    let mut used = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    used[0] = true;
    q.push_back(v[0][0]);
    used[v[0][0]] = true;
    loop {
        let mut update = false;
        let t = q.front().unwrap().clone();
        for &nxt in v[t].iter() {
            if used[nxt] {
                continue;
            }
            used[nxt] = true;
            q.push_front(nxt);
            update = true;
            break;
        }
        if !update {
            break;
        }
    }

    loop {
        let mut update = false;
        let t = q.back().unwrap().clone();
        for &nxt in v[t].iter() {
            if used[nxt] {
                continue;
            }
            used[nxt] = true;
            q.push_back(nxt);
            update = true;
            break;
        }
        if !update {
            break;
        }
    }

    println!("{}", q.len());
    while let Some(idx) = q.pop_front() {
        print!("{} ", idx + 1);
    }
    println!("");
}