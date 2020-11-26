use proconio::input;
use proconio::marker::Chars;

fn main()
{
    input! {
    s:Chars
    }
    let md = 1000_000_007i64;

    let (mut a, mut b, mut c) = (0i64, 0i64, 0i64);
    let mut p = 1i64;
    for t in s {
        match t {
            'A' => { a = (a + p) % md; }
            'B' => { b = (b + a) % md; }
            'C' => { c = (c + b) % md; }
            _ => {
                c = (c * 3 + b) % md;
                b = (b * 3 + a) % md;
                a = (a * 3 + p) % md;
                p = (p * 3) % md;
            }
        };
    }

    println!("{}", c);
}