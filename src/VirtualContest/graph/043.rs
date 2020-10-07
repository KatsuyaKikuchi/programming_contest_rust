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
    let mut count = vec![0; n];
    for (a, b) in edge {
        v[b].push(a);
        count[a] += 1;
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        if count[i] == 0 {
            q.push_back(i);
        }
    }

    let check = q.len() > 1;
    let mut ans = Vec::new();
    while let Some(idx) = q.pop_front() {
        ans.push(idx);
        for &nxt in v[idx].iter() {
            count[nxt] -= 1;
            if count[nxt] == 0 {
                q.push_back(nxt);
            }
        }
    }

    ans.reverse();
    for i in ans {
        println!("{}", i + 1);
    }
    println!("{}", if check { 1 } else { 0 });
}