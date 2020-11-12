use proconio::input;
use proconio::marker::Chars;

fn main()
{
    input! {
    n:usize,
    s:Chars,
    spell:[Chars;n],
    }

    let size = s.len();
    let md = 1000000007;
    let mut dp = vec![0; size + 1];
    dp[0] = 1;
    for i in 0..size {
        if dp[i] == 0 {
            continue;
        }
        for j in 0..spell.len() {
            let t = &spell[j];
            if i + t.len() > size {
                continue;
            }
            let mut same = true;
            for k in 0..t.len() {
                if s[i + k] != t[k] {
                    same = false;
                    break;
                }
            }
            if same {
                dp[i + t.len()] = (dp[i + t.len()] + dp[i]) % md;
            }
        }
    }
    println!("{}", dp[size]);
}