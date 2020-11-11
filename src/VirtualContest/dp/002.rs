use proconio::input;

fn calc(n: usize) -> usize {
    let mut n = n;
    let mut ret = n;
    while n > 0 {
        ret += n % 10;
        n /= 10;
    };
    ret
}

fn main()
{
    input! {
    n:usize,
    }
    let mut dp = vec![1; n + 1];
    for i in 1..n {
        let nxt = calc(i);
        if nxt > n {
            continue;
        }
        dp[nxt] += dp[i];
    }
    println!("{}", dp[n]);
}