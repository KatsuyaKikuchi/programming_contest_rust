use proconio::input;

fn main()
{
    loop {
        input! {
        s:String,
        m:usize,
        v:[usize;m],
        }
        if s.find("-") != None {
            break;
        }

        let sum = v.iter().sum::<usize>() % s.len();
        println!("{}", format!("{}{}", &s[sum..], &s[0..sum]));
    }
}