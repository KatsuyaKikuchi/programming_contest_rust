use proconio::input;

fn main()
{
    loop
    {
        input! {
        s:String,
        }

        let ans = s.chars().map(|c| {
            (c as i32) - ('0' as i32)
        }).sum::<i32>();
        if ans == 0 {
            break;
        }

        println!("{}", ans);
    }
}