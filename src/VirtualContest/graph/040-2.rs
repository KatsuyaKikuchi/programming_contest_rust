use proconio::input;
use proconio::marker::Usize1;
use std::cmp::min;

fn main()
{
    input! {
    n:usize,
    m:usize,
    l:i64,
    edge:[(Usize1,Usize1,i64);m],
    q:usize,
    query:[(Usize1,Usize1);q],
    }

    let inf = 1i64 << 60;
    let mut v = vec![vec![inf; n]; n];
    for (a, b, c) in edge {
        v[a][b] = c;
        v[b][a] = c;
    }
    for i in 0..n {
        v[i][i] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                v[i][j] = min(v[i][j], v[i][k] + v[k][j]);
            }
        }
    }

    let mut dist = vec![vec![inf; n]; n];
    for i in 0..n {
        for j in 0..n {
            if v[i][j] <= l {
                dist[i][j] = 1;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }

    for (s, t) in query {
        let ans = if dist[s][t] == inf {
            -1
        } else {
            dist[s][t] - 1
        };
        println!("{}", ans);
    }
}