use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;
use std::cmp::{Reverse, min};

fn main()
{
    input! {
    (n,m,k):(usize,usize,usize),
    edge:[(Usize1,Usize1,i64);m],
    a:[(usize,i64);n]
    }

    let inf = 1i64 << 60;
    let mut cost = vec![vec![inf; k + 1]; n];
    let mut v = vec![vec![]; n];

    for (a, b, c) in edge {
        v[a].push((b, c));
        v[b].push((a, c));
    }

    cost[0][0] = 0;
    let mut q = BinaryHeap::new();
    q.push(Reverse((0, 0, 0)));

    while let Some(Reverse((cst, idx, num))) = q.pop()
    {
        if cost[idx][num] < cst {
            continue;
        }

        let mut cst = cst;
        let mut num = num;
        loop {
            for &(nxt, dst) in v[idx].iter()
            {
                let cst = cst + dst;
                if cost[nxt][num] <= cst {
                    continue;
                }
                cost[nxt][num] = cst;
                q.push(Reverse((cst, nxt, num)));
            }
            num = min(num + a[idx].0, k);
            cst += a[idx].1;

            if cost[idx][num] <= cst {
                break;
            }
            cost[idx][num] = cst;
        }
    }

    let ans = if cost[n - 1][k] == inf {
        -1
    } else {
        cost[n - 1][k]
    };

    println!("{}", ans);
}