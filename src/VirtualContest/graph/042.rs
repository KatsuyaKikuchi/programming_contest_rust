use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;
use std::cmp::max;

fn main()
{
    input! {
    n:usize,
    edge:[(Usize1,Usize1);n-1],
    }

    if n == 1 {
        println!("First");
        return;
    }

    let mut v = vec![vec![]; n];
    for (a, b) in edge {
        v[a].push(b);
        v[b].push(a);
    }

    let mut s = 0;
    for i in 0..n {
        if v[i].len() == 1 {
            s = i;
            break;
        }
    }

    let mut len = 0;
    let mut q = VecDeque::new();
    q.push_back((s, 0));
    let mut used = vec![false; n];
    used[s] = true;
    while let Some((idx, dist)) = q.pop_front() {
        for &nxt in v[idx].iter() {
            if used[nxt] {
                continue;
            }
            used[nxt] = true;
            let dist = dist + 1;
            if len < dist {
                s = nxt;
            }
            len = max(len, dist);
            q.push_back((nxt, dist));
        }
    }
    let mut q = VecDeque::new();
    q.push_back((s, 0));
    let mut used = vec![false; n];
    used[s] = true;
    while let Some((idx, dist)) = q.pop_front() {
        for &nxt in v[idx].iter() {
            if used[nxt] {
                continue;
            }
            used[nxt] = true;
            let dist = dist + 1;
            len = max(len, dist);
            q.push_back((nxt, dist));
        }
    }

    let ans = if len % 3 == 1 {
        "Second"
    } else {
        "First"
    };
    println!("{}", ans);
}