use proconio::input;

fn main()
{
    loop {
        input! {
        h:i32,
        w:i32,
        }

        if h == 0 && w == 0 {
            break;
        }
        for _h in 0..h {
            for _w in 0..w {
                let c = if (_h + _w) % 2 == 0 {
                    '#'
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!("");
        }
        println!("");
    }
}