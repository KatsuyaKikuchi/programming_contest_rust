use proconio::input;
use std::collections::VecDeque;

struct Node {
    edge: Vec<usize>
}

impl Node {
    fn new() -> Self {
        Node {
            edge: Vec::new(),
        }
    }
}

fn dist(s: &String, t: &String) -> i32
{
    let mut ret = 0;
    let v = s.chars().collect::<Vec<char>>();
    let u = t.chars().collect::<Vec<char>>();
    let n = s.len();
    for i in 0..n {
        if v[i] != u[i] {
            ret += 1;
        }
    }
    ret
}

fn main()
{
    input! {
    first:String,
    last:String,
    n:usize,
    mut s:[String;n],
    }

    if first == last {
        println!("{}", 0);
        println!("{}", first);
        println!("{}", last);
        return;
    }

    s.push(first.clone());
    s.push(last.clone());
    s.sort();
    s.dedup();

    let mut v = (0..s.len()).map(|_| Node::new()).collect::<Vec<Node>>();
    for i in 0..s.len() {
        for j in i + 1..s.len() {
            if dist(&s[i], &s[j]) <= 1 {
                v[i].edge.push(j);
                v[j].edge.push(i);
            }
        }
    }

    let inf = 1i64 << 60;
    let mut cost = vec![inf; s.len()];
    let start_index = s.iter().position(|t| t.clone() == first).unwrap();
    let last_index = s.iter().position(|t| t.clone() == last).unwrap();

    cost[start_index] = 0;
    let mut q = VecDeque::new();
    q.push_back(start_index);
    while let Some(idx) = q.pop_front() {
        let dist = cost[idx] + 1;
        for &nxt in v[idx].edge.iter() {
            if cost[nxt] <= dist {
                continue;
            }
            cost[nxt] = dist;
            q.push_back(nxt);
        }
    }

    if cost[last_index] == inf {
        println!("{}", -1);
        return;
    }

    let mut ans = Vec::new();
    let mut q = VecDeque::new();
    q.push_back(last_index);
    while let Some(idx) = q.pop_front() {
        ans.push(s[idx].clone());
        for &nxt in v[idx].edge.iter() {
            if cost[idx] - 1 != cost[nxt] {
                continue;
            }
            q.push_back(nxt);
            break;
        }
    }

    ans.reverse();
    println!("{}", ans.len() - 2);
    for t in ans {
        println!("{}", t);
    }
}