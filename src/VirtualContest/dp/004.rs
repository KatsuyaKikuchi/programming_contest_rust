use proconio::input;
use proconio::marker::Chars;

fn main()
{
    input! {
    (n,k):(usize,usize),
    (r,s,p):(i64,i64,i64),
    c:Chars
    }

    let mut ans = 0;
    for i in 0..k {
        let mut idx = i;
        let mut num = 0;
        while idx < n {
            if idx >= k && c[idx] != c[idx - k] {
                num = 0;
            }
            num += 1;
            if num % 2 == 1 {
                ans += match c[idx] {
                    'p' => s,
                    'r' => p,
                    's' => r,
                    _ => 0,
                };
            }
            idx += k;
        }
    }
    println!("{}", ans);
}