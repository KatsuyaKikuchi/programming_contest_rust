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