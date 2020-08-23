use proconio::input;

fn main()
{
    input! {
    n:i32,
    v:[(char,i32);n],
    }

    let mark = vec!['S', 'H', 'C', 'D'];
    for &m in mark.iter() {
        let mut number = 1;
        while number <= 13
        {
            let mut exist = false;
            for &(c, i) in v.iter() {
                if c != m || i != number {
                    continue;
                }
                exist = true;
                break;
            }
            if !exist {
                println!("{} {}", m, number);
            }
            number += 1;
        }
    }
}