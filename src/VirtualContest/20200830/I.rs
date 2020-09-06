use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;
use std::cmp::max;

struct Node {
    to: Vec<usize>,
    from: Vec<usize>,
    count: usize,
    dist: u32,
}

impl Node {
    fn new() -> Self {
        Node {
            to: Vec::new(),
            from: Vec::new(),
            count: 0,
            dist: 0,
        }
    }
}

fn main()
{
    input! {
    n:usize,
    m:usize,
    edge:[(Usize1,Usize1);m],
    }

    let mut v = (0..n).map(|_| Node::new()).collect::<Vec<Node>>();
    for &(a, b) in edge.iter() {
        v[a].to.push(b);
        v[b].from.push(a);
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        if v[i].to.len() == 0 {
            q.push_back(i);
        }
    }

    while let Some(idx) = q.pop_front() {
        let dist = v[idx].dist + 1;
        for i in 0..v[idx].from.len() {
            let nxt = v[idx].from[i];
            v[nxt].dist = max(v[nxt].dist, dist);
            v[nxt].count += 1;

            if v[nxt].count == v[nxt].to.len() {
                q.push_back(nxt);
            }
        }
    }

    let ans = v.into_iter().map(|e| e.dist).max().unwrap();
    println!("{}", ans);
}