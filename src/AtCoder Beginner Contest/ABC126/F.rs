use proconio::input;

fn main()
{
    input! {
    m:i64,
    k:i64,
    }

    let mx = (1i64 << m) - 1;

    let ans =
        if k > mx || (mx <= 1 && k > 0) {
            vec![-1]
        } else if k > 0 {
            vec![k].into_iter()
                .chain((0..=mx).filter(|&i| i != k))
                .chain(vec![k])
                .chain((0..=mx).filter(|&i| i != k).rev())
                .collect::<Vec<i64>>()
        } else {
            (0..=mx).chain((0..=mx).rev()).collect::<Vec<i64>>()
        };

    for &i in ans.iter()
    {
        print!("{} ", i);
    }
    println!("");
}