use proconio::input;

fn main() {
    input! {
    n:usize,
    a:[i64;n],
    }

    let mut ans: i64 = a.iter().map(|&x| x.abs()).sum();
    let min = a.iter().map(|&x| x.abs()).min().unwrap();
    let cnt = a.into_iter().filter(|&x| x < 0).count();

    if cnt % 2 == 1 { ans -= 2 * min; }
    println!("{}", ans);
}