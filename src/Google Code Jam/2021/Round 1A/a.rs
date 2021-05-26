use std::cmp::min;

fn convert(value: i64) -> Vec<i64> {
    let mut ret = vec![];
    let mut value = value;
    while value > 0 {
        ret.push(value % 10);
        value /= 10;
    }
    ret.reverse();
    ret
}

fn next(base: &Vec<i64>, target: &Vec<i64>) -> Vec<i64> {
    let n = base.len();
    let m = target.len();
    let mut ret = target.clone();
    if n >= m {
        for i in 0..m {
            if base[i] == target[i] {
                continue;
            }
            if base[i] < target[i] {
                // 残り0うめ
                while ret.len() < n {
                    ret.push(0);
                }
                return ret;
            }
            if base[i] > target[i] {
                // 残り0うめ
                while ret.len() <= n {
                    ret.push(0);
                }
                return ret;
            }
        }

        // targetの前半が同じになっている
        if n == m {
            ret.push(0);
        } else {
            let mut val = 1;
            let mut v = vec![];
            for i in (m..n).rev() {
                let nxt = val + base[i];
                val = nxt / 10;
                v.push(nxt % 10);
            }
            if val > 0 {
                v.push(0);
            }
            v.reverse();
            for i in 0..v.len() {
                ret.push(v[i]);
            }
        }
    }
    ret
}

fn solve(v: &Vec<i64>) -> usize {
    let mut ret = 0;
    let n = v.len();
    let mut value = convert(v[0]);

    for i in 1..n {
        let now = convert(v[i]);
        value = next(&value, &now);
        ret += value.len() - now.len();
        //println!("{:?} {}", value, ret);
    }
    ret
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let q = sc.read::<usize>();
    for case in 0..q {
        let n = sc.read::<usize>();
        let mut v = vec![0; n];
        for i in 0..n {
            v[i] = sc.read();
        }
        println!("Case #{}: {}", case + 1, solve(&v));
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