use proconio::input;
use std::ops::Mul;

struct Matrix {
    row: usize,
    column: usize,
    values: Vec<Vec<i64>>,
}

impl Matrix {
    fn new(row: usize, column: usize) -> Matrix {
        Matrix {
            row: row,
            column: column,
            values: vec![vec![0; column]; row],
        }
    }

    fn print(&self) {
        for i in 0..self.row {
            for j in 0..self.column {
                print!("{} ", self.values[i][j]);
            }
            println!("");
        }
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, other: Self) -> Self {
        let mut ret = Self::new(self.row, other.column);
        for i in 0..self.row {
            for j in 0..other.column {
                for n in 0..self.column {
                    ret.values[i][j] += self.values[i][n] * other.values[n][j];
                }
            }
        }
        ret
    }
}

fn main()
{
    input! {
    n:usize,
    m:usize,
    l:usize,
    v:[[i64;m];n],
    u:[[i64;l];m],
    }

    let mut mat0 = Matrix::new(n, m);
    let mut mat1 = Matrix::new(m, l);
    for i in 0..n {
        for j in 0..m {
            mat0.values[i][j] = v[i][j];
        }
    }
    for i in 0..m {
        for j in 0..l {
            mat1.values[i][j] = u[i][j];
        }
    }

    let ans = mat0 * mat1;
    ans.print();
}