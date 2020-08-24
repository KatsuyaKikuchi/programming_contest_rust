use proconio::input;

fn main()
{
    input! {
    r:usize,
    c:usize,
    mut v:[[i32;c];r],
    }
    v.push(vec![0; c]);
    for i in 0..c {
        for j in 0..r {
            v[r][i] += v[j][i];
        }
    }
    for i in 0..v.len() {
        v[i].push(0);
        for j in 0..c {
            v[i][c] += v[i][j];
        }
    }


    for t in v.iter() {
        println!("{:?}", t);
    }
}