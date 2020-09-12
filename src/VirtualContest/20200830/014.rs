use proconio::input;
use proconio::marker::Usize1;
use std::cmp::min;

fn main()
{
    input! {
    n:usize,
    m:usize,
    edge:[(Usize1,Usize1,i64);m],
    }

    let inf = 1i64 << 60;
    let mut cost = vec![vec![inf; n]; n];
    for i in 0..n {
        cost[i][i] = 0;
    }

    let mut v = Vec::new();
    for &(a, b, c) in edge.iter() {
        if a == 0 || b == 0 {
            v.push((a, b, c));
            continue;
        }
        cost[a][b] = c;
        cost[b][a] = c;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                cost[i][j] = min(cost[i][k] + cost[k][j], cost[i][j]);
            }
        }
    }

    let mut ans = inf;
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            ans = min(v[i].2 + v[j].2 + cost[v[j].1][v[i].1], ans);
        }
    }

    if ans >= inf {
        ans = -1;
    }
    println!("{}", ans);
}