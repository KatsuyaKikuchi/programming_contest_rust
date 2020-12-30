use proconio::input;

fn main()
{
    input! {
    (n,k):(usize,i64),
    mat:[[i32;n];n]
    }

    let mut dp = vec![vec![vec![0; n]; 70]; n];
    let _mod = 1000000007i64;
    for i in 0..n {
        for j in 0..n {
            if mat[i][j] == 1 {
                dp[i][0][j] = 1;
            }
        }
    }
    for i in 0..61 {
        for idx in 0..n {
            for a in 0..n {
                for b in 0..n {
                    dp[idx][i + 1][a] += (dp[b][i][a] * dp[idx][i][b]) % _mod;
                    dp[idx][i + 1][a] %= _mod;
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        let mut v = Vec::new();
        v.push(vec![0; n]);
        v[0][i] = 1;
        for i in 0..60 {
            if (k & (1i64 << i)) == 0 {
                continue;
            }
            let tmp = v.last().unwrap();
            let mut vv = vec![0i64; n];
            for a in 0..n {
                for b in 0..n {
                    vv[b] += (dp[a][i][b] * tmp[a]) % _mod;
                    vv[b] %= _mod;
                }
            }
            v.push(vv);
        }

        ans += v.last().unwrap().iter().fold(0i64, |s, &v| (s + v) % _mod);
        ans %= _mod;
    }

    println!("{}", ans);
}