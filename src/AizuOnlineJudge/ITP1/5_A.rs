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

        for _ in 0..h {
            println!("{}", "#".repeat(w));
        }
        println!("");
    }
}