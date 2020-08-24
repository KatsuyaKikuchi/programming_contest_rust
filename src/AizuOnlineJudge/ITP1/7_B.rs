use proconio::input;

fn main()
{
    loop {
        input! {
        n:i32,
        x:i32,
        }
        if n == 0 && x == 0 {
            break;
        }
        let mut ans = 0;
        for i in 1..=n {
            for j in (i + 1)..=n {
                for k in (j + 1)..=n {
                    if i + j + k == x {
                        ans += 1;
                    }
                }
            }
        }
        println!("{}", ans);
    }
}