use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
use std::cmp::max;

fn main()
{
    input! {
    n:usize,
    edge:[Chars;n]
    }

    let mut ans = -1;
    'main: for i in 0..n {
        // 頂点iをv0に配置
        let mut v = vec![-1; n];
        let mut q = VecDeque::new();
        q.push_back((i, 0));
        v[i] = 0;

        let mut mx = 0;
        while let Some((idx, cst)) = q.pop_front() {
            let cst = cst + 1;
            for j in 0..n {
                if edge[idx][j] == '0' {
                    continue;
                }

                if v[j] >= 0 {
                    if cst % 2 != v[j] {
                        // 2分グラフでない
                        continue 'main;
                    }
                    continue;
                }
                v[j] = cst % 2;
                mx = max(mx, cst + 1);
                q.push_back((j, cst));
            }
        }
        ans = max(ans, mx);
    }

    println!("{}", ans);
}