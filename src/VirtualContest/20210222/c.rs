use proconio::input;

fn main()
{
    input! {
    n:usize,
    v:[i64;n]
    }
    let mut ans = vec![0; n];
    for i in (0..n).rev() {
        let idx = if (n - 1 - i) % 2 == 0 {
            (n - 1 - i) / 2
        } else {
            n - 1 - (n - 1 - i) / 2
        };
        ans[idx] = v[i];
    }

    for i in ans {
        print!("{} ", i);
    }
    println!("")
}