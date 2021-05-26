fn gcd(a: usize, b: usize) -> usize {
    if a < b {
        return gcd(b, a);
    }
    let c = a % b;
    if c == 0 {
        return b;
    }
    gcd(b, c)
}

fn solve(v: &[(Vec<char>, i32)]) {
    let n = v.len();
    let q = v[0].0.len();
    let len = 2i32.pow(q as u32);
    let mut ques = vec![];
    for bit in 0..len {
        let mut t = vec![];
        for i in 0..q {
            if ((bit >> i) & 1) == 1 {
                t.push('T');
            } else {
                t.push('F');
            }
        }

        let mut enable = true;
        for i in 0..n {
            let mut score = 0;
            for j in 0..q {
                if v[i].0[j] == t[j] {
                    score += 1;
                }
            }
            if score != v[i].1 {
                enable = false;
                break;
            }
        }
        if enable {
            ques.push(t.clone());
        }
    }

    let mut mx = (0, 0);
    for bit in 0..len {
        let mut ans = vec![];
        for i in 0..q {
            if ((bit >> i) & 1) == 1 {
                ans.push('T');
            } else {
                ans.push('F');
            }
        }

        let mut score = 0;
        for i in 0..ques.len() {
            for j in 0..q {
                if ans[j] == ques[i][j] {
                    score += 1;
                }
            }
        }

        if mx.0 <= score {
            mx = (score, bit);
        }
    }

    let bit = mx.1;
    for i in 0..q {
        if ((bit >> i) & 1) == 1 {
            print!("T");
        } else {
            print!("F");
        }
    }
    let w = mx.0;
    let z = ques.len();
    let g = gcd(w, z);
    println!(" {}/{}", w / g, z / g);
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let t = sc.read::<usize>();
    for case in 0..t {
        print!("Case #{}: ", case + 1);
        let (n, q) = (sc.read::<usize>(), sc.read::<usize>());
        let mut v = vec![];
        for _ in 0..n {
            let x = (sc.chars(), sc.read::<i32>());
            v.push(x);
        }

        solve(&v);
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