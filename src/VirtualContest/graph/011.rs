use proconio::input;
use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;

fn main()
{
    input! {
    h:usize,
    w:usize,
    v:[[i64;w];h],
    }

    let inf = 1i64 << 60;
    let dt: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut cost = vec![vec![vec![inf; w]; h]; 3];
    let s = [(h - 1, 0), (0, w - 1), (h - 1, w - 1)];
    for (idx, &g) in s.iter().enumerate() {
        cost[idx][g.0][g.1] = 0;
        let mut q = BinaryHeap::new();
        q.push(Reverse((0, g)));
        while let Some(Reverse((cst, (x, y)))) = q.pop() {
            for &(dx, dy) in dt.iter() {
                let (nx, ny) = (dx + x as isize, dy + y as isize);
                if nx < 0 || nx >= h as isize || ny < 0 || ny >= w as isize {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                let dist = cst + v[nx][ny];
                if cost[idx][nx][ny] <= dist {
                    continue;
                }
                cost[idx][nx][ny] = dist;
                q.push(Reverse((dist, (nx, ny))));
            }
        }
    }

    let mut ans = inf;
    for i in 0..h {
        for j in 0..w {
            let mut sum = 0;
            for k in 0..3 {
                sum += cost[k][i][j];
            }
            sum -= v[i][j] * 2;

            ans = min(ans, sum);
        }
    }
    println!("{}", ans);
}