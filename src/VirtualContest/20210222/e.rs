use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main()
{
    input! {
    (n,m):(usize,usize),
    s:Chars
    }

    let inf = 1_000_000_000_000_000i64;
    let mut dp = vec![inf; n + 1];
    dp[n] = 0;
    let mut q = VecDeque::new();
    q.push_back((0, n));
    for i in (0..n).rev() {
        while let Some(&(_, idx)) = q.front() {
            if idx - i <= m {
                break;
            }
            q.pop_front();
        }

        if s[i] == '1' {
            continue;
        }

        if let Some(&(v, _)) = q.front()
        {
            dp[i] = v + 1;
        }
        while let Some(&(v, _)) = q.back() {
            if v < dp[i] {
                break;
            }
            q.pop_back();
        }
        q.push_back((dp[i], i));
    }

    if dp[0] >= inf {
        println!("{}", -1);
        return;
    }
    let mut ans = Vec::new();
    let mut nxt = dp[0] - 1;
    let mut prev = 0;
    for i in 0..(n + 1) {
        if dp[i] == nxt {
            ans.push(i - prev);
            prev = i;
            nxt = dp[i] - 1;
        }
    }
    for v in ans {
        print!("{} ", v);
    }
    println!("");
}