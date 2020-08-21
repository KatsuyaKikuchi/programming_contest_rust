use proconio::input;

fn main()
{
    let mut case = 1;
    loop {
        input! {x:i32}

        if x == 0 {
            break;
        }

        println!("Case {}: {}", case, x);
        case += 1;
    }
}