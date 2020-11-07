use proconio::input;
use proconio::marker::Chars;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main()
{
    input! {
    (n,m):(usize,usize),
    k:usize,
    d:Chars,
    board:[Chars;n]
    }
    let inf = 1i64 << 60;
    let mut dist = vec![vec![inf; m]; n];
    let (mut x0, mut y0) = (0, 0);
    let (mut x1, mut y1) = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if board[i][j] == 'S' {
                x0 = i;
                y0 = j;
            }
            if board[i][j] == 'G' {
                x1 = i;
                y1 = j;
            }
        }
    }
    let mut nxt = vec![vec![k; 4]; k];
    for i in 0..k {
        let dir = match d[i] {
            'R' => 0,
            'D' => 1,
            'L' => 2,
            'U' => 3,
            _ => k
        };
        nxt[i][dir] = i;
    }
    for i in (0..2 * k).rev() {
        let idx = i % k;
        let n = (idx + 1) % k;
        for j in 0..4 {
            if nxt[idx][j] == k {
                nxt[idx][j] = nxt[n][j];
            }
        }
    }

    let mut q = BinaryHeap::new();
    let dt = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    q.push(Reverse((0, (x0, y0))));
    while let Some(Reverse((dst, (x, y)))) = q.pop() {
        if dist[x][y] < dst {
            continue;
        }
        let idx = (dst as usize) % k;
        for (i, &(dx, dy)) in dt.iter().enumerate() {
            let (x, y) = (x as i32 + dx, y as i32 + dy);
            if x < 0 || y < 0 {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if x >= n || y >= m {
                continue;
            }
            let n = nxt[idx][i];
            if n == k || board[x][y] == '#' {
                continue;
            }
            let d = if n < idx {
                n + k - idx
            } else {
                n - idx
            };
            let dst = dst + 1 + d as i64;
            if dist[x][y] <= dst {
                continue;
            }
            dist[x][y] = dst;
            q.push(Reverse((dst, (x, y))));
        }
    }

    let ans = if dist[x1][y1] == inf {
        -1
    } else {
        dist[x1][y1]
    };
    println!("{}", ans);
}