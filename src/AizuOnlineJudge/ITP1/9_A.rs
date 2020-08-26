
fn main()
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let mut v: Vec<String> = Vec::new();
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        if buf.find("END_OF_TEXT") != None {
            break;
        }
        for t in buf.to_lowercase().trim().split_whitespace() {
            v.push(t.to_string());
        }
    }

    let mut ans = 0;
    for t in v {
        if s == t {
            ans += 1;
        }
    }
    println!("{}", ans);
}