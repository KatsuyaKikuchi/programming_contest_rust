use proconio::input;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<i32>,
    size: usize,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).map(|i| i).collect(),
            rank: vec![1; size],
            size: size,
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

    fn unit(&mut self, n: usize, m: usize) -> usize {
        let n = self.find(n);
        let m = self.find(m);
        if n == m {
            return n;
        }
        if self.rank[n] > self.rank[m] {
            self.parent[m] = n;
            n
        } else {
            self.parent[n] = m;
            if self.rank[n] == self.rank[m] {
                self.rank[n] += self.rank[m];
            }
            m
        }
    }

    fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut parent_buf = vec![0; self.size];
        let mut group_size = vec![0; self.size];
        for i in 0..self.size {
            parent_buf[i] = self.find(i);
            group_size[parent_buf[i]] += 1;
        }

        let mut ret = vec![Vec::new(); self.size];
        for i in 0..self.size {
            ret[i].reserve(group_size[i]);
        }

        for i in 0..self.size {
            ret[parent_buf[i]].push(i);
        }

        ret
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect()
    }
}

// return max index that f(value,v[index])==true
fn lower_bound<T>(value: T, v: &Vec<T>, f: Box<dyn Fn(&T, &T) -> bool>) -> Option<usize> {
    let n = v.len();
    if n == 0 {
        return None;
    }
    if !f(&value, &v[0]) {
        return None;
    }

    let (mut ret, mut ng) = (0, n);
    while (ng - ret) > 1 {
        let mid = (ng + ret) / 2;
        if f(&value, &v[mid]) {
            ret = mid;
        } else {
            ng = mid;
        }
    }
    Some(ret)
}

fn main()
{
    input! {
    n:usize,
    mut pos:[(i64,i64);n],
    }

    let mut x = Vec::new();
    for &(a, _) in pos.iter() {
        x.push(a);
    }
    x.sort();
    x.dedup();

    let mut uf = UnionFind::new(x.len());
    let mut y = vec![vec![]; x.len()];
    pos.sort_by(|(_, y0), (_, y1)| y0.cmp(y1));
    for i in 0..n {
        let idx0 = lower_bound(pos[i].0, &x,
                               Box::new(|a, b| a >= b)).unwrap();
        y[idx0].push(pos[i].1);

        if i == n - 1 || pos[i].1 != pos[i + 1].1 {
            continue;
        }
        let idx1 = lower_bound(pos[i + 1].0, &x,
                               Box::new(|a, b| a >= b)).unwrap();
        uf.unit(idx0, idx1);
    }

    let mut ans: i64 = -(n as i64);
    let group = uf.groups();
    for g in group {
        let mut v = Vec::new();
        for &i in g.iter() {
            v.append(&mut y[i]);
        }
        v.sort();
        v.dedup();
        ans += (v.len() as i64) * (g.len() as i64);
    }

    println!("{}", ans);
}