use proconio::input;

fn main()
{
    input! {
    n:usize,
    c:[i64;n],
    }
    let mut ans = 1000;
    let mut stock = 0;

    if c[1] >= c[0] {
        stock += ans / c[0];
        ans %= c[0];
    }
    for i in 1..(n - 1) {
        if c[i - 1] <= c[i] && c[i] >= c[i + 1] {
            ans += stock * c[i];
            stock = 0;
        }
        if c[i - 1] >= c[i] && c[i] <= c[i + 1] {
            stock += ans / c[i];
            ans %= c[i];
        }
    }
    ans += stock * c[n - 1];

    println!("{}", ans);
}