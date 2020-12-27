use proconio::input;
use proconio::marker::Usize1;
use std::cmp::min;
use std::collections::BTreeSet;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<i32>,
    count: Vec<usize>,
    size: usize,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).map(|i| i).collect(),
            rank: vec![1; size],
            count: vec![1; size],
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
            self.count[n] += self.count[m];
            n
        } else {
            self.parent[n] = m;
            self.count[m] += self.count[n];
            if self.rank[n] == self.rank[m] {
                self.rank[n] += self.rank[m];
            }
            m
        }
    }

    fn size(&mut self, n: usize) -> usize {
        let n = self.find(n);
        self.count[n]
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

fn main()
{
    input! {
    (n,m,q):(usize,usize,usize),
    mut edge:[(Usize1,Usize1,i64);m],
    query:[usize;q]
    }

    edge.sort_by_key(|&(_, _, c)| c);

    let mut uf = UnionFind::new(n);
    let mut nums = vec![0; n + 1];
    nums[1] = n;
    let inf = 1i64 << 60;
    let mut ans = vec![inf; n + 1];
    ans[1] = 0;

    let mut set = BTreeSet::new();
    set.insert(1);

    for (a, b, c) in edge {
        if uf.same(a, b) {
            continue;
        }
        let (s0, s1) = (uf.size(a), uf.size(b));
        nums[s0] -= s0;
        if nums[s0] == 0 {
            set.remove(&s0);
        }
        nums[s1] -= s1;
        if nums[s1] == 0 {
            set.remove(&s1);
        }
        set.insert(s0 + s1);
        nums[s0 + s1] += s0 + s1;

        let idx = set.iter().min().unwrap().clone();
        ans[idx] = min(ans[idx], c);
        uf.unit(a, b);
    }

    for i in (0..n).rev() {
        ans[i] = min(ans[i], ans[i + 1]);
    }

    for x in query {
        if x > n || ans[x] == inf {
            println!("trumpet")
        } else {
            println!("{}", ans[x]);
        }
    }
}