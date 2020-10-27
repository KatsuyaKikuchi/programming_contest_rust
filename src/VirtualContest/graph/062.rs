use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;
use std::cmp::min;

fn count(t: Vec<usize>, v: &Vec<Vec<usize>>, k: i32) -> usize {
    let n = v.len();
    let inf = 1 << 30;
    let mut dist = vec![inf; n];
    let mut q = VecDeque::new();
    for i in t {
        q.push_back(i);
        dist[i] = 0;
    }
    while let Some(idx) = q.pop_front() {
        let cst = dist[idx] + 1;
        if cst > k / 2 {
            continue;
        }
        for &nxt in v[idx].iter() {
            if dist[nxt] <= cst {
                continue;
            }
            dist[nxt] = cst;
            q.push_back(nxt);
        }
    };
    n - dist.into_iter().filter(|&x| x < inf).count()
}

fn main()
{
    input! {
    n: usize,
    k: i32,
    edge:[(Usize1, Usize1); n - 1]
    }

    let mut v = vec![vec![]; n];
    for (a, b) in edge {
        v[a].push(b);
        v[b].push(a);
    }

    let mut ans = n + 1;
    for i in 0..n {
        if k % 2 == 1 {
            for &nxt in v[i].iter() {
                if nxt > i {
                    continue;
                }
                let t = vec![i, nxt];
                ans = min(ans, count(t, &v, k));
            }
        } else {
            let t = vec![i];
            ans = min(ans, count(t, &v, k));
        }
    }

    println!("{}", ans);
}