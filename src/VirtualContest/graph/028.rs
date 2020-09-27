use proconio::input;
use proconio::marker::Usize1;
use std::cmp::{min, max};
use std::collections::VecDeque;

fn get_index(a: usize, b: usize, n: usize) -> usize {
    let (a, b, n) = (min(a, b) as i32, max(a, b) as i32, n as i32);
    let mut ret = a * (2 * (n - 1) - (a - 1)) / 2;
    ret += b - (a + 1);

    ret as usize
}

fn main() {
    input! {
    n:usize,
    t:[[Usize1;n-1];n]
    }

    let mut node = vec![vec![]; n * (n - 1) / 2];
    let mut count = vec![0; n * (n - 1) / 2];
    for (i, v) in t.iter().enumerate() {
        for j in 0..(n - 2) {
            let (idx, nxt) = (get_index(i, v[j], n), get_index(i, v[j + 1], n));
            node[idx].push(nxt);
            count[nxt] += 1;
        }
    }

    let mut q = VecDeque::new();
    for (i, &x) in count.iter().enumerate() {
        if x == 0 {
            q.push_back((i, 1));
        }
    }

    let inf = 1i64 << 60;
    let mut dist = vec![inf; n * (n - 1) / 2];
    while let Some((idx, dst)) = q.pop_front() {
        dist[idx] = dst;
        let dst = dst + 1;
        for &nxt in node[idx].iter() {
            count[nxt] -= 1;
            if count[nxt] == 0 {
                q.push_back((nxt, dst));
            }
        }
    }

    let mx = dist.into_iter().max().unwrap();
    let ans = if mx == inf {
        -1
    } else {
        mx
    };

    println!("{}", ans);
}