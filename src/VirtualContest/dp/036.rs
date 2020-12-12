use proconio::input;

fn count(n: i64) -> i64 {
    let mut x = n;
    let mut s = vec![];
    while x > 0
    {
        let m = x % 10;
        s.push(m as usize);
        x /= 10;
    }
    if s.len() == 0 {
        return 0;
    }
    s.reverse();
    let mut dp = vec![vec![vec![0; 2]; 10]; s.len()];
    for i in 0..s[0] {
        dp[0][i][0] = 1;
    }
    dp[0][s[0]][1] = 1;
    dp[0][4][0] = 0;
    dp[0][4][1] = 0;
    dp[0][9][1] = 0;
    for i in 1..s.len() {
        for k in 0..10 {
            for j in 0..10 {
                dp[i][j][0] += dp[i - 1][k][0];
            }
            for j in 0..s[i] {
                dp[i][j][0] += dp[i - 1][k][1];
            }
            dp[i][s[i]][1] += dp[i - 1][k][1];
        }
        for j in 0..2 {
            dp[i][4][j] = 0;
            dp[i][9][j] = 0;
        }
    }

    let mut ret = n;
    for i in 0..10 {
        ret -= dp[s.len() - 1][i][0] + dp[s.len() - 1][i][1];
    }
    ret + 1
}

fn main()
{
    input! {
    (a,b):(i64,i64)
    }

    let ans = count(b) - count(a - 1);
    println!("{}", ans);
}