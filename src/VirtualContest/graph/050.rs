use proconio::input;
use proconio::marker::Usize1;

fn main()
{
    input! {
    n:usize,
    edge:[(Usize1,Usize1);n-1],
    }

    let mut v = vec![vec![]; n];
    for (a, b) in edge {
        v[a].push(b);
        v[b].push(a);
    }

    let mut s = vec![0; n + 1];
    for i in (0..n).rev() {
        let mut t = 1;
        for &idx in v[i].iter() {
            if idx > i {
                t -= 1;
            }
        }
        s[i] = s[i + 1] + t;
    }

    let mut sum = s.iter().sum::<i64>();

    let mut ans = 0;
    for i in (0..n).rev() {
        ans += sum;
        let mut t = Vec::new();
        for &idx in v[i].iter() {
            if idx > i {
                continue;
            }
            t.push(idx);
        }
        t.sort();
        let start = if let Some(idx) = t.pop() {
            idx as i64
        } else {
            -1
        };
        sum -= (i as i64) - start;
        sum += t.iter().map(|&x| (x as i64) + 1).sum::<i64>();
    }

    println!("{}", ans);
}