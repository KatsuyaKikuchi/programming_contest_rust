use proconio::input;
use proconio::marker::Usize1;
use std::collections::{VecDeque, BinaryHeap};
use std::cmp::Reverse;

fn main()
{
    input! {
    n:usize,
    m:usize,
    edge:[(Usize1,Usize1,i64);m],
    }

    let mut node = vec![vec![]; n];
    for (i, &(a, b, c)) in edge.iter().enumerate() {
        node[a].push((b, c, i));
        node[b].push((a, c, i));
    }

    let inf = 1i64 << 60;
    let mut used = vec![false; m];
    for i in 0..n {
        let mut dist = vec![inf; n];
        dist[i] = 0;
        let mut q = BinaryHeap::new();
        q.push(Reverse((0, i)));
        while let Some(Reverse((cost, idx))) = q.pop() {
            if dist[idx] < cost {
                continue;
            }
            for &(nxt, c, _) in node[idx].iter() {
                let cst = c + cost;
                if dist[nxt] <= cst {
                    continue;
                }
                dist[nxt] = cst;
                q.push(Reverse((cst, nxt)));
            }
        }

        for j in (i + 1)..n {
            let mut q = VecDeque::new();
            q.push_back(j);
            while let Some(idx) = q.pop_front() {
                for &(nxt, cst, index) in node[idx].iter() {
                    if dist[idx] != dist[nxt] + cst {
                        continue;
                    }
                    q.push_back(nxt);
                    used[index] = true;
                }
            }
        }
    }

    let ans = used.into_iter().filter(|&n| !n).count();
    println!("{}", ans);
}