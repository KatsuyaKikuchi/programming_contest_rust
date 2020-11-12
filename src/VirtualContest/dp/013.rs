use proconio::input;
use proconio::marker::Chars;

fn main()
{
    input! {
    s:Chars,
    t:Chars,
    }

    let (n, m) = (s.len(), t.len());
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for (i, &c0) in s.iter().enumerate() {
        for (j, &c1) in t.iter().enumerate() {
            if c0 == c1 {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            }
            dp[i + 1][j + 1] = vec![dp[i + 1][j + 1], dp[i][j + 1], dp[i + 1][j]].iter().max().unwrap().clone();
        }
    }
    let mut value = dp[n][m];
    let mut ans = Vec::new();

    let (mut h, mut w) = (n, m);
    while value > 0 {
        while dp[h][w - 1] == value {
            w -= 1;
        }
        while dp[h - 1][w] == value {
            h -= 1;
        }
        w -= 1;
        h -= 1;
        ans.push(s[h]);
        value -= 1;
    }

    for c in ans.iter().rev() {
        print!("{}", c);
    }
    println!("");
}