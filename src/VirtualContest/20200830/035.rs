use proconio::input;
use std::cmp::{min, Reverse, max};
use std::collections::BinaryHeap;
use ordered_float::OrderedFloat;

fn main()
{
    input! {
    n:usize,
    edge:[(f64,f64,i32,i32);n],
    }

    let mut v = vec![vec![]; n];
    for i in 0..n {
        for j in i + 1..n {
            let d = ((edge[i].0 - edge[j].0).powf(2.0) + (edge[i].1 - edge[j].1).powf(2.0)).sqrt();
            let s = min(edge[i].2, edge[j].3) as f64;
            let t = min(edge[i].3, edge[j].2) as f64;

            v[i].push((d / s, j));
            v[j].push((d / t, i));
        }
    }

    let inf = 1000000000f64;
    let mut dist = vec![inf; n];
    dist[0] = 0.0;

    let mut q = BinaryHeap::new();
    q.push(Reverse((OrderedFloat(0.0), 0)));

    while let Some(Reverse((dst, idx))) = q.pop()
    {
        if dist[idx] < dst.0 {
            continue;
        }

        for &(cost, nxt) in v[idx].iter() {
            let dst = dst.0 + cost;
            if dist[nxt] <= dst {
                continue;
            }
            dist[nxt] = dst;
            q.push(Reverse((OrderedFloat(dst), nxt)));
        }
    }

    let mut ans = OrderedFloat(0.0);
    let mut dist = dist.into_iter()
        .map(|d| OrderedFloat(d))
        .collect::<Vec<OrderedFloat<f64>>>();
    dist.sort();

    for i in 1..n {
        let diff = (n - i - 1) as f64;
        ans = max(OrderedFloat(dist[i].0 + diff), ans);
    }
    println!("{}", ans.0);
}