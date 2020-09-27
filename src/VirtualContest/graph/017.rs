use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

struct Node {
    to: Vec<usize>,
    from: Vec<usize>,
}

impl Node {
    fn new() -> Self
    {
        Node {
            to: Vec::new(),
            from: Vec::new(),
        }
    }
}

fn main() {
    input! {
    n:usize,
    m:usize,
    edge:[(Usize1,Usize1);n+m-1],
    }

    let mut node = (0..n).map(|_| Node::new()).collect::<Vec<Node>>();
    for (a, b) in edge {
        node[a].to.push(b);
        node[b].from.push(a);
    }

    let mut cnt = vec![0; n];
    let mut ans = vec![0; n];

    let mut q = VecDeque::new();
    let root = node.iter().position(|n| n.from.len() == 0).unwrap();
    q.push_back(root);

    while let Some(idx) = q.pop_front() {
        for i in 0..node[idx].to.len() {
            let nxt = node[idx].to[i];
            cnt[nxt] += 1;
            if cnt[nxt] == node[nxt].from.len() {
                ans[nxt] = idx + 1;
                q.push_back(nxt);
            }
        }
    }

    for i in ans {
        println!("{}", i);
    }
}