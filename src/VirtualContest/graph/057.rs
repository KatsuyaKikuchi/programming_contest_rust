use proconio::input;
use std::cmp::{ min, Reverse};
use std::collections::BinaryHeap;

struct Solve {
    md: i64,
}

impl Solve {
    fn new(m: i64) -> Self {
        Solve {
            md: m
        }
    }

    fn solve(&self, g: &Vec<Vec<(usize, i64)>>) -> i64 {
        let inf = 1i64 << 60;
        let n = g.len();
        let mut dist = vec![vec![inf; self.md as usize]; n];
        dist[0][0] = 0;

        let mut q = BinaryHeap::new();
        q.push(Reverse((0, 0)));
        while let Some(Reverse((dst, idx))) = q.pop() {
            if dist[idx][(dst % self.md) as usize] < dst {
                continue;
            }
            for &(nxt, cst) in g[idx].iter() {
                let dst = dst + cst;
                if dist[nxt][(dst % self.md) as usize] <= dst {
                    continue;
                }
                dist[nxt][(dst % self.md) as usize] = dst;
                if nxt != n - 1 {
                    q.push(Reverse((dst, nxt)));
                }
            }
        }

        dist[n - 1][0]
    }
}

fn main()
{
    input! {
(n, m): (usize, usize),
edge: [(usize, usize,i64); m]
}

    let mut v = vec![vec![]; n];
    for (a, b, c) in edge {
        v[a].push((b, c));
        v[b].push((a, c));
    }

    let solve_f = Solve::new(4);
    let solve_s = Solve::new(7);

    let ans = min(solve_f.solve(&v), solve_s.solve(&v));

    println!("{}", ans);
}