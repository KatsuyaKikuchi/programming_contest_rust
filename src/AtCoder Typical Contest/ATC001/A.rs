use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main()
{
    input! {
    h:usize,
    w:usize,
    s:[Chars;h],
    }

    let mut u = vec![vec![false; w]; h];

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    for i in 0..h {
        match s[i].iter().position(|&c| c == 's') {
            Some(width) => {
                q.push_back((i, width));
                break;
            }
            None => continue,
        };
    }

    let (x, y) = (vec![0i32, 1, 0, -1], vec![1i32, 0, -1, 0]);
    while let Some((_h, _w)) = q.pop_front() {
        if s[_h][_w] == 'g' {
            println!("Yes");
            return;
        }
        for i in 0..4 {
            let (nx, ny) = (x[i] + _h as i32, y[i] + _w as i32);
            if nx < 0 || nx >= h as i32 || ny < 0 || ny >= w as i32 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if s[nx][ny] == '#' || u[nx][ny] {
                continue;
            }
            u[nx][ny] = true;
            q.push_back((nx, ny));
        }
    }
    println!("No");
}