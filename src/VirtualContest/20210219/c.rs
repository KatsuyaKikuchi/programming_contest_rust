use proconio::input;
use proconio::marker::Chars;

fn solve(s: &Vec<char>) -> bool {
    let mut v = s.clone();
    v.sort();
    let mut count = vec![0; 10];
    for &n in v.iter() {
        count[n.to_digit(10).unwrap() as usize] += 1;
    }

    'main: for n in 1..1000 {
        if n % 8 != 0 {
            continue;
        }
        let mut cnt = vec![0; 10];
        for x in n.to_string().chars() {
            cnt[x.to_digit(10).unwrap() as usize] += 1;
        }

        for i in 0..10 {
            if cnt[i] > count[i] {
                continue 'main;
            }
        }
        if cnt.iter().sum::<i32>() == 3 || cnt.iter().sum::<i32>() == count.iter().sum::<i32>() {
            return true;
        }
    }
    false
}

fn main()
{
    input! {
    s:Chars,
    }
    let ans = if solve(&s) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}