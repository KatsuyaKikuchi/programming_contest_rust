use proconio::input;
use proconio::marker::Usize1;
use std::cmp::{Ordering, min};
use std::mem::swap;

fn next_permutation<T>(mut v: Vec<T>) -> Option<Vec<T>>
    where T: Ord {
    let mut t = Vec::new();
    t.push(v.pop().unwrap());
    while let Some(mut lst) = v.pop() {
        if lst.cmp(t.last().unwrap()) != Ordering::Less {
            t.push(lst);
            continue;
        }
        t.sort();
        for i in 0..t.len() {
            if t[i] > lst {
                swap(&mut t[i], &mut lst);
                break;
            }
        }
        v.push(lst);
        return Some(v.into_iter().chain(t).collect::<Vec<T>>());
    }
    None
}

fn main()
{
    input! {
    n:usize,
    m:usize,
    r:usize,
    mut v:[Usize1;r],
    edge:[(Usize1,Usize1,i64);m]
    }
    v.sort();

    let inf = 1i64 << 60;

    let mut cost = vec![vec![inf; n]; n];
    for i in 0..n {
        cost[i][i] = 0;
    }
    for &(a, b, c) in edge.iter() {
        cost[a][b] = c;
        cost[b][a] = c;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                cost[i][j] = min(cost[i][j], cost[i][k] + cost[k][j]);
            }
        }
    }

    let mut ans = inf;
    loop {
        let mut sum = 0;
        for i in 0..(v.len() - 1) {
            sum += cost[v[i]][v[i + 1]];
        }
        ans = min(ans, sum);
        if let Some(nxt) = next_permutation(v) {
            v = nxt;
        } else {
            break;
        }
    }

    println!("{}", ans);
}