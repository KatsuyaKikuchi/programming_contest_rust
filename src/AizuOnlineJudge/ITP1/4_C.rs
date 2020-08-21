use proconio::input;

fn main()
{
    loop {
        input! {
        a:i32,
        op:char,
        b:i32
        }

        let ans = match op {
            '+' => Some(a + b),
            '-' => Some(a - b),
            '/' => Some(a / b),
            '*' => Some(a * b),
            _ => None
        };

        if let Some(x) = ans {
            println!("{}", x);
        } else {
            break;
        }
    }
}