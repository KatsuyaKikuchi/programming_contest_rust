use proconio::input;
use std::cmp::min;

fn main() {
    input! {
    (n,k):(usize,usize),
    points:[(i64,i64);n]
    }

    let mut x = points
        .iter()
        .map(|(x, _)| x.clone())
        .collect::<Vec<i64>>();
    x.sort();
    x.dedup();

    let mut ans = 4_000_000_000_000_000_000i64;
    for i in 0..x.len() {
        for j in i..x.len() {
            let len = x[j] - x[i];
            let mut p = points
                .iter()
                .filter(|(xv, _)| xv.clone() <= x[j] && xv.clone() >= x[i])
                .map(|v| v.clone())
                .collect::<Vec<(i64, i64)>>();
            p.sort_by(|(_, b), (_, d)| b.cmp(d));
            for i in (k - 1)..p.len() {
                ans = min(ans, len * (p[i].1 - p[i - (k - 1)].1));
            }
        }
    }
    println!("{}", ans);
}