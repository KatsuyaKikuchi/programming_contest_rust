use proconio::input;

fn check(mut x: i32) -> bool {
    if x % 3 == 0
    {
        return true;
    }
    while x > 0 {
        if x % 10 == 3 {
            return true;
        }
        x /= 10;
    }
    false
}

fn main()
{
    input! {
    n:i32,
    }

    for i in 1..=n {
        let x = i;
        let exist = check(x);

        if exist {
            print!(" {}", i);
        }
    }
    println!("");
}