use proconio::input;

fn main()
{
    loop {
        input! {
        h:usize,
        w:usize,
        }

        if h == 0 && w == 0
        {
            break;
        }

        for _h in 0..h {
            for _w in 0..w {
                if _h == 0 || _w == 0 || _h == h - 1 || _w == w - 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }
}