use proconio::input;
use proconio::marker::Usize1;

pub struct UnionFind {
    n: usize,
    parent_or_size: Vec<i32>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            n: size,
            parent_or_size: vec![-1; size],
        }
    }

    pub fn find(&mut self, idx: usize) -> usize {
        assert!(idx < self.n);
        if self.parent_or_size[idx] < 0 {
            return idx;
        }
        self.parent_or_size[idx] = self.find(self.parent_or_size[idx] as usize) as i32;
        self.parent_or_size[idx] as usize
    }

    pub fn same(&mut self, idx0: usize, idx1: usize) -> bool {
        self.find(idx0) == self.find(idx1)
    }

    pub fn unit(&mut self, idx0: usize, idx1: usize) -> usize {
        let (idx0, idx1) = (self.find(idx0), self.find(idx1));
        if idx0 == idx1 {
            return idx0;
        }
        if -self.parent_or_size[idx0] < -self.parent_or_size[idx1] {
            return self.unit(idx1, idx0);
        }
        self.parent_or_size[idx0] += self.parent_or_size[idx1];
        self.parent_or_size[idx1] = idx0 as i32;
        idx0
    }

    pub fn size(&mut self, idx: usize) -> usize {
        let idx = self.find(idx);
        -self.parent_or_size[idx] as usize
    }


    pub fn group(&mut self) -> Vec<Vec<usize>> {
        let mut parent_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            parent_buf[i] = self.find(i);
            group_size[parent_buf[i]] += 1;
        }
        let mut result = vec![Vec::new(); self.n];
        for i in 0..self.n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            result[parent_buf[i]].push(i);
        }

        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}

fn main()
{
    input! {
    (n,m):(usize,usize),
    edge:[(Usize1,Usize1);m]
    }

    let mut ans = 0;
    for i in 0..m {
        let mut uf = UnionFind::new(n);
        let filter_edge = edge
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx.clone() != i)
            .map(|(_, v)| v);
        for &(a, b) in filter_edge {
            uf.unit(a, b);
        }
        if uf.size(0) == n {
            continue;
        }
        ans += 1;
    }
    println!("{}", ans);
}