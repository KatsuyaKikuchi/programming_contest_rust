use proconio::input;

fn main()
{
    input! {
    n:usize,
    v:[(String,String);n],
    }

    let (mut a, mut b) = (0, 0);

    for (s, t) in v {
        if s > t {
            a += 3;
        } else if s < t {
            b += 3;
        } else {
            a += 1;
            b += 1;
        }
    }

    println!("{} {}", a, b);
}