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
    let mut dist = vec![vec![inf; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for &(a, b, c) in edge.iter() {
        dist[a][b] = min(dist[a][b], c);
        dist[b][a] = min(dist[b][a], c);
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }

    input! {
    k:usize,
    edge:[(Usize1,Usize1,i64);k],
    }

    for &(a, b, c) in edge.iter() {
        dist[a][b] = min(dist[a][b], c);
        dist[b][a] = min(dist[b][a], c);
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = min(dist[i][j], dist[i][a] + dist[a][b] + dist[b][j]);
                dist[i][j] = min(dist[i][j], dist[i][b] + dist[b][a] + dist[a][j]);
            }
        }

        let sum = dist.iter().map(|v| v.iter().sum::<i64>()).sum::<i64>();
        println!("{}", sum / 2);
    }
}