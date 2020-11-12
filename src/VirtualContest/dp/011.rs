use proconio::input;

fn main()
{
    input! {
    n:usize,
    p:[usize;n]
    }

    let mut v = vec![false; 100 * 100 + 5];
    v[0] = true;
    for t in p {
        for i in (0..v.len()).rev() {
            if i + t >= v.len() {
                continue;
            }
            if v[i] {
                v[i + t] = true;
            }
        }
    }

    let ans = v.into_iter().filter(|&b| b).count();
    println!("{}", ans);
}