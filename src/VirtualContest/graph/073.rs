use proconio::input;
use proconio::marker::Usize1;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main()
{
    input! {
    (h,w,x):(usize,usize,usize),
    s:(Usize1,Usize1),
    g:(Usize1,Usize1),
    map:[[usize;w];h],
    c:[i64;x]
    }

    let mut v = vec![Vec::new(); h * w];
    let dt = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    for i in 0..h {
        for j in 0..w {
            let idx = i * w + j;
            for &(dx, dy) in dt.iter() {
                let (x, y) = (i as i32 + dx, j as i32 + dy);
                if x < 0 || y < 0 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if x >= h || y >= w {
                    continue;
                }
                let nxt = x * w + y;
                let cost = if map[i][j] == map[x][y] || map[x][y] == 0 {
                    0
                } else {
                    c[map[x][y] - 1]
                };
                v[idx].push((cost, nxt));
            }
        }
    }

    let inf = 1i64 << 60;
    let (s, g) = (s.0 * w + s.1, g.0 * w + g.1);
    let mut dist = vec![inf; h * w];
    let mut q = BinaryHeap::new();
    dist[s] = 0;
    q.push(Reverse((0, s)));
    while let Some(Reverse((cost, idx))) = q.pop() {
        if dist[idx] < cost {
            continue;
        }
        for &(cst, nxt) in v[idx].iter() {
            let cost = cost + cst;
            if dist[nxt] <= cost {
                continue;
            }
            dist[nxt] = cost;
            q.push(Reverse((cost, nxt)));
        }
    }

    println!("{}", dist[g]);
}