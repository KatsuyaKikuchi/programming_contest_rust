use proconio::input;

fn main()
{
    input! {
    n:usize,
    mut r:[(i64,i64);n],
    mut b:[(i64,i64);n]
    }
    r.sort();
    b.sort();

    let mut used = vec![false; n];
    let mut ans = 0;
    for &(x, y) in b.iter() {
        let mut index = -1;
        for (i, &(xr, yr)) in r.iter().enumerate() {
            if used[i] {
                continue;
            }
            if xr > x || yr > y {
                continue;
            }

            if index < 0 || r[index as usize].1 < yr {
                index = i as i32;
            }
        }
        if index < 0 {
            continue;
        }
        used[index as usize] = true;
        ans += 1;
    }

    println!("{}", ans);
}