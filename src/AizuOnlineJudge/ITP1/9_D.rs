use proconio::input;

fn main()
{
    input! {
    mut s:String,
    n:i32,
    }

    for _ in 0..n {
        input! {
        q:String,
        a:usize,
        b:usize,
        }

        if q == "print" {
            println!("{}", &s[a..=b]);
        } else if q == "reverse" {
            let t = s.clone();
            let (x, y, z) = (&t[0..a], &t[a..=b].chars().rev().collect::<String>(), &t[b + 1..]);
            s = format!("{}{}{}", x, y, z);
        } else if q == "replace" {
            input! {
            c:String,
            }
            let t = s.clone();
            let (x, y) = (&t[0..a], &t[b + 1..]);
            s = format!("{}{}{}", x, c, y);
        }
    }
}