use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main()
{
    input! {
    n:usize,
    m:usize,
    edge:[(Usize1,Usize1,i64);m],
    }

    let mut v0 = vec![vec![]; n];
    let mut v1 = vec![vec![]; n];
    for &(a, b, _) in edge.iter() {
        v0[a].push(b);
        v1[b].push(a);
    }
    let mut count = vec![0; n];
    let mut used = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    used[0] = true;
    while let Some(idx) = q.pop_front() {
        count[idx] += 1;
        for &nxt in v0[idx].iter() {
            if used[nxt] {
                continue;
            }
            used[nxt] = true;
            q.push_back(nxt);
        }
    }
    let mut used = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back(n - 1);
    used[n - 1] = true;
    while let Some(idx) = q.pop_front() {
        count[idx] += 1;
        for &nxt in v1[idx].iter() {
            if used[nxt] {
                continue;
            }
            used[nxt] = true;
            q.push_back(nxt);
        }
    }

    let inf = 1i64 << 60;
    let mut score = (0..n).map(|i| if i == 0 { 0 } else { inf }).collect::<Vec<i64>>();

    for _ in 0..n + 5 {
        let mut update = false;

        for (a, b, c) in edge.iter().map(|&(a, b, c)| (a, b, -c)) {
            if count[a] < 2 || count[b] < 2 {
                continue;
            }
            if score[a] + c >= score[b] {
                continue;
            }
            score[b] = score[a] + c;
            update = true;
        }

        if !update {
            println!("{}", -score[n - 1]);
            return;
        }
    }
    println!("inf");
}