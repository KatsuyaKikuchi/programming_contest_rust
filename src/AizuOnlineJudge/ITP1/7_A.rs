use proconio::input;

fn get_grade(m: i32, f: i32, r: i32) -> Option<char> {
    match (m, f, r) {
        (-1, -1, -1) => None,
        (-1, _, _) | (_, -1, _) => Some('F'),
        _ if m + f >= 80 => Some('A'),
        _ if m + f >= 65 => Some('B'),
        _ if m + f >= 50 => Some('C'),
        _ if m + f >= 30 && r >= 50 => Some('C'),
        _ if m + f >= 30 => Some('D'),
        _ => Some('F'),
    }
}

fn main()
{
    loop {
        input! {
        (m,f,r):(i32,i32,i32)
        }

        if let Some(grade) = get_grade(m, f, r) {
            println!("{}", grade);
        } else {
            break;
        }
    }
}