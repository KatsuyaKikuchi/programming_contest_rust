use proconio::input;

fn count(a: &Vec<i64>, b: &Vec<i64>, value: i64) -> i64 {
    let mut count = 0;
    for &n in a.iter() {
        if (n * b[0] > value) {
            continue;
        }
        let (mut ok, mut ng) = (0, b.len());
        while (ng - ok) > 1 {
            let mid = (ok + ng) / 2;
            if b[mid] * n <= value {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        count += (ok + 1) as i64;
    }
    count
}

fn main()
{
    input! {
    n:usize,
    k:i64,
    mut a:[i64;n],
    mut b:[i64;n]
    }
    a.sort();
    b.sort();

    let inf = 1i64 << 60;
    let (mut ok, mut ng) = (0, inf);
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if count(&a, &b, mid) < k {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok + 1);
}