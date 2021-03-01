fn primes(n: usize) -> Vec<i32> {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; n + 1];
    for p in 2..=n {
        if !is_prime[p] {
            continue;
        }
        primes.push(p as i32);
        for j in (2 * p..).into_iter().step_by(p).take_while(|x| x <= &n) {
            is_prime[j] = false;
        }
    }
    primes
}

fn main()
{
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());
    let n = sc.read::<usize>();

    let prime = primes(55555);
    let mut ans = vec![];
    for p in prime {
        if p % 10 == 1 {
            ans.push(p);
        }
        if ans.len() == n {
            break;
        }
    }

    for a in ans {
        print!("{} ", a);
    }
    println!("");
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