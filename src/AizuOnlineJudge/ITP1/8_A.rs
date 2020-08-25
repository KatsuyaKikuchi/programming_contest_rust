fn main()
{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    /*
    let ans = buf.as_bytes().iter().map(|&c|
        {
            match c {
                b'A'..=b'Z' => (c - b'A' + b'a') as char,
                b'a'..=b'z' => (c - b'a' + b'A') as char,
                _ => c as char,
            }
        }).collect::<String>();
     */

    let ans = buf.chars().map(|c| {
        match c {
            'A'..='Z' => c.to_ascii_lowercase(),
            'a'..='z' => c.to_ascii_uppercase(),
            _ => c,
        }
    }).collect::<String>();

    println!("{}", ans);
}