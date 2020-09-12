use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main()
{
    input! {
    n:usize,
    m:usize,
    edge:[(Usize1,Usize1);m],
    (s,t):(Usize1,Usize1),
    }
    let mut node = vec![vec![]; n];
    for (a, b) in edge {
        node[a].push(b);
    }

    let inf = 1i64 << 60;
    let mut cost = vec![vec![inf; 3]; n];
    cost[s][0] = 0;

    let mut q = VecDeque::new();
    q.push_back((s, 0, 0));

    while let Some((idx, dist, cnt)) = q.pop_front()
    {
        let cnt = (cnt + 1) % 3;
        let dist = if cnt == 1 {
            dist + 1
        } else {
            dist
        };

        for &nxt in node[idx].iter() {
            if cost[nxt][cnt] <= dist {
                continue;
            }
            cost[nxt][cnt] = dist;
            q.push_back((nxt, dist, cnt));
        }
    }

    let ans = if cost[t][0] == inf {
        -1
    } else {
        cost[t][0]
    };

    println!("{}", ans);
}