use proconio::input;
use proconio::marker::Usize1;

fn main()
{
    input! {
    n:usize,
    m:i32,
    arr:[(Usize1,Usize1);m],
    }

    let mut v: Vec<i32> = vec![0; n + 1];
    for a in arr {
        v[a.0] += 1;
        v[a.1 + 1] -= 1;
    }

    for i in 0..n {
        v[i + 1] += v[i];
    }

    let ans = v.into_iter().filter(|&x| x == m).count();
    println!("{}", ans);
}