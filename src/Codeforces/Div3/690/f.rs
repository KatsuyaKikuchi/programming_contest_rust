use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn solve(v: &mut [(i64, i64)]) -> usize {
    let n = v.len();
    let mut cnt = vec![n; n];
    let mut v = v.iter().enumerate().map(|(i, &(a, b))| (a, b, i)).collect::<Vec<_>>();
    v.sort();
    let mut q = BinaryHeap::new();
    let mut sum = 0;
    for &(l, r, i) in v.iter() {
        while let Some(&Reverse(val)) = q.peek() {
            if l <= val {
                break;
            }
            sum += 1;
            q.pop();
        }
        cnt[i] -= sum;
        q.push(Reverse(r));
    }
    v.sort_by(|(_, a, _), (_, b, _)| b.cmp(a));
    let mut sum = 0;
    let mut q = BinaryHeap::new();
    for &(l, r, i) in v.iter() {
        while let Some(&val) = q.peek() {
            if val <= r {
                break;
            }
            sum += 1;
            q.pop();
        }
        cnt[i] -= sum;
        q.push(l);
    }
    n - cnt.iter().max().unwrap().clone()
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let q = sc.read();
    for _ in 0..q {
        let n = sc.read();
        let mut v = Vec::new();
        for _ in 0..n {
            let t = (sc.read::<i64>(), sc.read::<i64>());
            v.push(t);
        }

        println!("{}", solve(&mut v));
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        self.1.write_all(s.to_string().as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn usize0(&mut self) -> usize {
        self.read::<usize>() - 1
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}