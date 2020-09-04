use std::cmp::Ordering;
use std::mem::swap;

fn next_permutation<T>(mut v: Vec<T>) -> Option<Vec<T>>
    where T: Ord {
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