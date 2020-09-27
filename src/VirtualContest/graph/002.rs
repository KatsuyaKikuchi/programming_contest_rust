use proconio::input;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem::swap;
use proconio::marker::Usize1;

fn next_permutation<T>(mut v: Vec<T>) -> Option<Vec<T>>
    where T: Ord + Debug {
    let mut t = Vec::new();
    t.push(v.pop().unwrap());
    while let Some(mut lst) = v.pop() {
        if lst.cmp(t.last().unwrap()) != Ordering::Less {
            t.push(lst);
            continue;
        }
        t.sort();
        for i in 0..t.len() {
            if t[i] > lst {
                swap(&mut t[i], &mut lst);
                break;
            }
        }
        v.push(lst);
        return Some(v.into_iter().chain(t).collect::<Vec<T>>());
    }
    None
}

fn main()
{
    input! {
    n:usize,
    m:usize,
    edge:[(Usize1,Usize1);m],
    }
    let mut v = (0..n).collect::<Vec<usize>>();

    let mut ans = 0;
    loop {
        let mut exist = true;
        for i in 0..n - 1 {
            let (x, y) = (v[i], v[i + 1]);
            if let Some(_) = edge.iter()
                .find(|&t| (t.0, t.1) == (x, y) || (t.1, t.0) == (x, y)) {
                continue;
            } else {
                exist = false;
                break;
            }
        }
        if exist {
            ans += 1;
        }
        if let Some(nxt) = next_permutation(v) {
            v = nxt;
            if v[0] > 0 {
                break;
            }
        } else {
            break;
        }
    }
    println!("{}", ans);
}