use proconio::input;

fn main()
{
    input! {
    (n,k,l):(usize,usize,usize)
    }
    let MOD = 1000000007i64;
    let num = 8usize.pow(k as u32);
    let mask = num - 1;
    let mut dp = vec![vec![0; num]; n + 1];
    let menu = vec![(0, 0, 0), (0, 0, 1), (0, 1, 0), (0, 1, 1), (1, 0, 0), (1, 0, 1), (1, 1, 0), (1, 1, 1)];
    dp[0][0] = 1;
    for i in 0..n {
        for p in 1..8 {
            for j in 0..num {
                if dp[i][j] == 0 {
                    continue;
                }

                let nxt = (mask & (j << 3)) | p;
                let mut count = (0, 0, 0);
                for x in 0..k {
                    let p = nxt >> (3 * x);
                    count.0 += menu[p % 8].0;
                    count.1 += menu[p % 8].1;
                    count.2 += menu[p % 8].2;
                }
                if count.0 > l || count.1 > l || count.2 > l {
                    continue;
                }
                dp[i + 1][nxt] += dp[i][j];
                dp[i + 1][nxt] %= MOD;
            }
        }
    }

    let ans = dp[n].iter().fold(0i64, |sum, &x| (sum + x) % MOD);
    println!("{}", ans);
}