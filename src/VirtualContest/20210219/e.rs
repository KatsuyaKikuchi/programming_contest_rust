use proconio::input;

fn main()
{
    input! {
    (n,mut k):(usize,usize)
    }
    let max = (n - 1) * (n - 2) / 2;
    if max < k {
        println!("-1");
        return;
    }
    let mut edge = Vec::new();
    for i in 1..n {
        edge.push((1, i + 1));
    }
    'main: for i in 1..n {
        for j in i + 1..n {
            if k == max {
                break 'main;
            }
            k += 1;
            edge.push((i + 1, j + 1));
        }
    }

    println!("{}", edge.len());
    for (a, b) in edge {
        println!("{} {}", a, b);
    }
}