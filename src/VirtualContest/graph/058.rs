use proconio::input;
use proconio::marker::Usize1;
use std::cmp::min;

fn main()
{
    input! {
    (n,m,mut s):(usize,usize,usize),
    edge:[(Usize1,Usize1,usize,i64);m],
    ex:[(usize,i64);n]
    }

    let mx = 4000usize;
    s = min(mx - 1, s);

    let inf = 1i64 << 60;
    // 都市iに銀貨をj枚持った状態でたどり着くための最小時間
    let mut dist = vec![vec![inf; mx]; n];
    dist[0][s] = 0;
    loop {
        let mut update = false;
        for i in 0..n {
            for j in 0..mx {
                let nxt = min(mx - 1, j + ex[i].0);
                let time = dist[i][j] + ex[i].1;
                dist[i][nxt] = min(dist[i][nxt], time);
            }
        }

        for &(a, b, c, d) in edge.iter() {
            for i in c..mx {
                let nxt = i - c;
                let time = dist[a][i] + d;
                if dist[b][nxt] <= time {
                    continue;
                }
                dist[b][nxt] = time;
                update = true;
            }
            for i in c..mx {
                let nxt = i - c;
                let time = dist[b][i] + d;
                if dist[a][nxt] <= time {
                    continue;
                }
                dist[a][nxt] = time;
                update = true;
            }
        }

        if !update {
            break;
        }
    }

    for i in 1..n {
        let mut ans = inf;
        for j in 0..mx {
            ans = min(ans, dist[i][j]);
        }
        println!("{}", ans);
    }
}