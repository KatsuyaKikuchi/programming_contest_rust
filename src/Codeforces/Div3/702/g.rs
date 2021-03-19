use std::cmp::max;

fn solve(a: &[i64], x: &[i64]) -> Vec<i64> {
    let (n, m) = (a.len(), x.len());
    let mut ret = vec![0; m];
    let mut sum = 0;
    let mut mx = -1_000_000_000i64;
    for i in 0..n {
        sum += a[i];
        mx = max(mx, sum);
    }
    let mut v = Vec::new();
    for i in 0..m {
        if x[i] <= mx {
            v.push((x[i], i));
        } else if sum <= 0 {
            ret[i] = -1;
        } else {
            let cnt = (x[i] - mx + sum - 1) / sum;
            ret[i] += cnt * n as i64;
            v.push((x[i] - sum * cnt, i));
        }
    }
    v.sort();
    let mut idx = 0;
    let mut sum = 0;
    for i in 0..n {
        sum += a[i];
        while idx < v.len() {
            if v[idx].0 > sum {
                break;
            }
            ret[v[idx].1] += i as i64;
            idx += 1;
        }
    }
    ret
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let q = sc.read::<usize>();
    for _ in 0..q {
        let (n, m) = (sc.read(), sc.read());
        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = sc.read::<i64>();
        }
        let mut x = vec![0; m];
        for i in 0..m {
            x[i] = sc.read::<i64>();
        }
        let ans = solve(&a, &x);
        for i in 0..ans.len() {
            print!("{} ", ans[i]);
        }
        println!("");
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