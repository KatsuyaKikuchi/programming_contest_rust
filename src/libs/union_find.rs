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

// cfg でビルドターゲットを指定
// ターゲットの時のみビルド
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_find_test() {
        let mut uf = UnionFind::new(10);
        assert_eq!(uf.n, 10);
        assert_eq!(uf.parent_or_size.len(), 10);
        for i in 0..10 {
            assert_eq!(uf.parent_or_size[i], -1);
            assert!(uf.same(i, i));
        }

        uf.unit(0, 1);
        assert!(uf.same(0, 1));
        assert!(uf.same(1, 0));
        assert_eq!(uf.size(0), 2);
        uf.unit(2, 1);
        assert!(uf.same(0, 1));
        assert!(uf.same(0, 2));
        assert!(uf.same(1, 2));

        assert_eq!(uf.size(0), 3);
        assert_eq!(uf.size(0), 3);
        assert_eq!(uf.size(2), 3);
        for i in 3..10 {
            assert_eq!(uf.size(i), 1);
            assert_eq!(uf.same(i, 0), false);
        }

        assert_eq!(uf.unit(5, 7), 5);
        assert!(uf.same(5, 7));
        assert_eq!(uf.size(7), 2);

        assert_eq!(uf.group(), vec![vec![0, 1, 2], vec![3], vec![4], vec![5, 7], vec![6], vec![8], vec![9]]);

        assert_eq!(uf.unit(5, 2), 0);
        let v = vec![0, 1, 2, 5, 7];
        for &a in v.iter() {
            for &b in v.iter() {
                assert!(uf.same(a, b));
            }
        }
    }
}