use proconio::input;
use std::collections::VecDeque;
use std::cmp::min;

fn main()
{
    input! {
    (n,m,k):(usize,usize,i64),
    v:[i64;n]
    }
    let inf = 1i64 << 60;
    let mut dp = vec![inf; n + 1];
    let mut mx = vec![VecDeque::new(); m];
    let mut mn = vec![VecDeque::new(); m];
    dp[0] = 0;
    for i in 0..n {
        for j in 0..m {
            while let Some(&idx) = mx[j].front() {
                if (i - idx) <= j {
                    break;
                }
                mx[j].pop_front();
            }
            while let Some(&idx) = mx[j].back() {
                if v[idx] > v[i] {
                    break;
                }
                mx[j].pop_back();
            }
            while let Some(&idx) = mn[j].front() {
                if (i - idx) <= j {
                    break;
                }
                mn[j].pop_front();
            }
            while let Some(&idx) = mn[j].back() {
                if v[idx] < v[i] {
                    break;
                }
                mn[j].pop_back();
            }
            mx[j].push_back(i);
            mn[j].push_back(i);
            if i < j {
                continue;
            }
            let (&a, &b) = (mx[j].front().unwrap(), mn[j].front().unwrap());
            let cost = (v[a] - v[b]) * ((j + 1) as i64) + k;
            dp[i + 1] = min(dp[i + 1], dp[i - j] + cost);
        }
    }

    let ans = dp[n];
    println!("{}", ans);
}