use proconio::input;

fn main()
{
    input! {
    h:i64,
    w:i64,
    n:usize,
    v:[(i64,i64);n],
    }

    let mut ans = w * (h - 1) + h * (w - 1);
    let d = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for i in 0..n {
        'dt: for &(x, y) in d.iter() {
            let (nx, ny) = (v[i].0 + x, v[i].1 + y);
            if nx <= 0 || nx > h || ny <= 0 || ny > w {
                continue;
            }

            for j in 0..i {
                if v[j] == (nx, ny) {
                    continue 'dt;
                }
            }
            ans -= 1;
        }
    }

    println!("{}", ans);
}