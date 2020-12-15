use proconio::input;
use std::collections::VecDeque;

fn main()
{
    input! {
    n:usize,
    v:[usize;n]
    }

    let mut ans = vec![0; n];
    let mx = v.iter().max().unwrap().clone();
    let mut stack = VecDeque::new();
    stack.push_front((mx + 1, 0));
    for i in 0..n {
        while let Some(&(val, _)) = stack.front() {
            if val > v[i] {
                break;
            }
            stack.pop_front();
        }
        if let Some(&(_, idx)) = stack.front() {
            ans[i] += i - idx;
        }
        stack.push_front((v[i], i + 1));
    }
    let mut stack = VecDeque::new();
    stack.push_front((mx + 1, n));
    for i in (0..n).rev() {
        while let Some(&(val, _)) = stack.front() {
            if val > v[i] {
                break;
            }
            stack.pop_front();
        }
        if let Some(&(_, idx)) = stack.front() {
            ans[i] += idx - i - 1;
        }
        stack.push_front((v[i], i));
    }

    for i in ans {
        println!("{}", i);
    }
}