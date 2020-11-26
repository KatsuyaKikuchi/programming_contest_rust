use proconio::input;

fn main()
{
    input! {
    (n,k):(usize,usize),
    v:[usize;n]
    }

    let mut win = vec![false; k + 1];
    for i in 0..k {
        if win[i] {
            continue;
        }
        for &n in v.iter() {
            if i + n > k {
                break;
            }
            win[i + n] = true;
        }
    }

    let ans = if win[k] {
        "First"
    } else {
        "Second"
    };
    println!("{}", ans);
}