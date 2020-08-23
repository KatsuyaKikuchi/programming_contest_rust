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
    r:usize,
    c:usize,
    }

    let mut mat = Matrix::new(r, c);
    let mut v = Matrix::new(c, 1);

    for i in 0..r {
        input! {
        value:[i64;c],
        }
        for j in 0..c {
            mat.values[i][j] = value[j];
        }
    }

    for i in 0..c {
        input! {
        value:i64,
        }
        v.values[i][0] = value;
    }

    let m = mat * v;
    m.print();
}