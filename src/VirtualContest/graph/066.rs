use proconio::input;
use proconio::marker::{Usize1, Chars};
use std::collections::VecDeque;

fn main()
{
    input! {
    (h,w,k):(usize,usize,usize),
    (x0,y0):(Usize1,Usize1),
    (x1,y1):(Usize1,Usize1),
    board:[Chars;h]
    }

    let inf = 1i64 << 60;
    let mut dist = vec![vec![vec![inf; 4]; w]; h];
    let dt = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
    let mut q = VecDeque::new();
    for i in 0..4 {
        dist[x0][y0][i] = 0;
    }
    q.push_back(((x0, y0), 0));

    while let Some(((x, y), dst)) = q.pop_front() {
        let dst = dst + 1;
        for (i, &nxt) in dt.iter().enumerate() {
            for d in 0..(k as i32) {
                let (x, y) = (x as i32 + nxt.0 * (d + 1), y as i32 + nxt.1 * (d + 1));
                if x < 0 || y < 0 {
                    break;
                }
                let (x, y) = (x as usize, y as usize);
                if x >= h || y >= w {
                    break;
                }
                if board[x][y] == '@' || dist[x][y][i] <= dst {
                    break;
                }
                let &mn = dist[x][y].iter().min().unwrap();
                if mn < dst {
                    break;
                }
                dist[x][y][i] = dst;
                q.push_back(((x, y), dst));
            }
        }
    }

    let ans = if let Some(&value) = dist[x1][y1].iter().min() {
        if value == inf {
            -1
        } else {
            value
        }
    } else {
        -1
    };

    println!("{}", ans);
}