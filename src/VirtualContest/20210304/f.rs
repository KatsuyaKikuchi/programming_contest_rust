use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n: usize = sc.read();
    let mut v = vec![0i64; 3 * n];
    for i in 0..3 * n {
        v[i] = sc.read();
    }
    let mut left = 0;
    let mut q = BinaryHeap::new();
    for i in 0..n {
        left += v[i];
        q.push(Reverse(v[i]));
    }
    let mut sum = vec![0; n + 1];
    for i in 0..(n + 1) {
        sum[i] = left;
        if let Some(Reverse(val)) = q.pop() {
            if val < v[i + n] {
                q.push(Reverse(v[i + n]));
                left += v[i + n] - val;
            } else {
                q.push(Reverse(val));
            }
        }
    }
    let mut right = 0;
    let mut q = BinaryHeap::new();
    for i in 0..n {
        right += v[2 * n + i];
        q.push(v[2 * n + i]);
    }

    for i in (0..(n + 1)).rev() {
        sum[i] -= right;
        if let Some(val) = q.pop() {
            if val > v[n + i - 1] {
                q.push(v[n + i - 1]);
                right += v[n + i - 1] - val;
            } else {
                q.push(val);
            }
        }
    }

    println!("{}", sum.into_iter().max().unwrap());
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