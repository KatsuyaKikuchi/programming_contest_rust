use proconio::input;

fn main()
{
    input! {
    s:String,
    k:usize
    }
    let mut strs = Vec::new();
    for i in 0..s.len() {
        for j in 0..k {
            if i + j >= s.len() {
                break;
            }
            let str = &s[i..i + j + 1];
            strs.push(str);
        }
    }
    strs.sort();
    strs.dedup();
    println!("{}", strs[k - 1]);
}