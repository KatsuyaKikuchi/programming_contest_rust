use proconio::input;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).map(|i| i).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, n: usize) -> usize {
        if self.parent[n] == n {
            return n;
        };
        self.parent[n] = self.find(self.parent[n]);
        self.parent[n]
    }

    fn same(&mut self, n: usize, m: usize) -> bool {
        self.find(n) == self.find(m)
    }

    fn unit(&mut self, n: usize, m: usize) {
        let n = self.find(n);
        let m = self.find(m);
        if n == m {
            return;
        }
        if self.rank[n] > self.rank[m] {
            self.parent[m] = n;
        } else {
            self.parent[n] = m;
            if self.rank[n] == self.rank[m] {
                self.rank[n] += self.rank[m];
            }
        }
    }
}

fn rev(mut x: usize) -> usize {
    let mut ret = 0;
    while x > 0 {
        ret = ret * 10 + x % 10;
        x /= 10;
    }
    ret
}

fn calc(mut x: usize, mut y: usize) -> (usize, usize)
{
    if x < y {
        x = rev(x);
    } else {
        y = rev(y);
    };

    if x < y {
        y -= x;
    } else {
        x -= y;
    };

    (x, y)
}

fn main()
{
    input! {
    n:usize,
    m:usize,
    }

    let size = 999 * 999 + 1;
    let mut uf = UnionFind::new(size);

    for i in 0..999 {
        for j in 0..999 {
            let index = i * 999 + j + 1;
            let (x, y) = calc(i + 1, j + 1);

            let chain = if x == 0 || y == 0 {
                0
            } else {
                (x - 1) * 999 + (y - 1) + 1
            };
            uf.unit(index, chain);
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            let index = i * 999 + j + 1;
            if uf.same(0, index) {
                continue;
            }
            ans += 1;
        }
    }

    println!("{}", ans);
}