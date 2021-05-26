use std::cmp::max;

fn dfs(index: usize, sum: i128, mul: i128, p: &[(i128, i128)], max_ans: i128) -> i128 {
    if index >= p.len() {
        return max_ans;
    }
    let mut mul = mul;
    let mut sum = sum;
    let mut ret = max(max_ans, dfs(index + 1, sum, mul, &p, max_ans));
    let mut count = 0;
    while mul <= sum && count < p[index].1 {
        mul *= p[index].0;
        sum -= p[index].0;
        if mul == sum {
            ret = max(ret, mul);
            break;
        } else if mul > sum {
            break;
        } else {
            ret = max(ret, dfs(index + 1, sum, mul, &p, ret));
        }
        count += 1;
    }
    ret
}

fn solve(v: &[(i128, i128)]) -> i128 {
    let n = v.len();
    let mut sum = 0;
    for i in 0..n {
        sum += v[i].0 * v[i].1;
    }
    dfs(0, sum, 1, &v, 0)
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let q = sc.read::<usize>();
    for case in 0..q {
        let n = sc.read::<usize>();
        let mut p = vec![(0, 0); n];
        for i in 0..n {
            p[i] = (sc.read(), sc.read());
        }
        p.reverse();
        println!("Case #{}: {}", case + 1, solve(&p));
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