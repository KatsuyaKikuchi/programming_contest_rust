use proconio::input;
use proconio::marker::{Usize1, Chars};
use std::collections::VecDeque;

fn main()
{
    input! {
    r:usize,
    c:usize,
    (sx,sy):(Usize1,Usize1),
    (gx,gy):(Usize1,Usize1),
    map:[Chars;r],
    }

    let inf = 2u64 << 60;
    let mut dist = vec![vec![inf; c]; r];

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((sx, sy));
    dist[sx][sy] = 0;
    let (x, y) = ([0, 1, 0, -1], [1, 0, -1, 0]);
    while let Some((_r, _c)) = q.pop_front() {
        let cost = dist[_r][_c] + 1;
        for i in 0..4 {
            let (nx, ny) = (x[i] + _r as i32, y[i] + _c as i32);
            if nx < 0 || nx > r as i32 || ny < 0 || ny > c as i32 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if map[nx][ny] == '#' || dist[nx][ny] <= cost {
                continue;
            }
            dist[nx][ny] = cost;
            q.push_back((nx, ny));
        }
    }
    println!("{}", dist[gx][gy]);
}