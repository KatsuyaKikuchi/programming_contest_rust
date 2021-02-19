use proconio::input;

fn mod_pow(n: i64, r: i64, m: i64) -> i64 {
    let mut ret = 1;
    let mut p = n;
    let mut r = r;
    while r > 0 {
        if (r & 1) == 1 {
            ret = ret * p % m;
        }
        r >>= 1;
        p = (p * p) % m;
    }
    ret
}

fn main()
{
    input! {
    (n,k):(i64,usize)
    }

    let m = 10i64.pow(9) + 7;
    let mut sum = vec![0; k + 1];
    let mut ans = 0;
    for i in (1..=k).rev() {
        // gcdがiになるものいくつ？
        let count = (k / i) as i64;
        let mut s = mod_pow(count, n, m);
        let mut j = i;
        while j <= k {
            s = (m + s - sum[j]) % m;
            j += i;
        }
        ans += s * (i as i64) % m;
        ans %= m;
        sum[i] = s;
    }
    println!("{}", ans);
}