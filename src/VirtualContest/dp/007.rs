use proconio::input;

fn main()
{
    input! {
    n:usize,
    v:[i64;n]
    }
    let sum = v.iter().map(|x| x.abs()).sum::<i64>();
    let cnt = v.iter().fold(0, |cnt, &x| if x <= 0 { cnt + 1 } else { cnt });
    let mn = v.iter().map(|x| x.abs()).min().unwrap();

    let ans = if cnt % 2 == 1 {
        sum - mn * 2
    } else {
        sum
    };
    println!("{}", ans);
}