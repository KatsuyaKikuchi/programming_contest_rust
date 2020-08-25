fn main()
{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    buf.to_ascii_lowercase();

    let mut ans = vec![0; 26];
    for i in buf.chars() {
        let index = match i {
            'a'..='z' => (i as i32) - ('a' as i32),
            _ => -1,
        };
        if index >= 0 {
            ans[index as usize] += 1;
        }
    }

    for i in 0..ans.len() {
        println!("{}", ans[i]);
    }
}