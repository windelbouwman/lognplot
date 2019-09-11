use std::ops::{Index, IndexMut};

pub struct Matrix {
    rows: usize,
    columns: usize,
    elements: Vec<f64>,
}

impl Matrix {
    fn new(rows: usize, columns: usize) -> Self {
        let elements = vec![0.0_f64; rows * columns];
        Self {
            rows,
            columns,
            elements,
        }
    }

    pub fn identity(s: usize) -> Self {
        let mut m = Self::new(s, s);
        for x in 0..s {
            m[(x, x)] = 1.0;
        }
        m
    }

    pub fn print(&self) {
        println!("[============]");
        for row in 0..self.rows {
            for column in 0..self.columns {
                print!("{},", self[(row, column)]);
            }
            println!("");
        }
        println!("[============]")
    }
}

type MatrixIndex = (usize, usize);

impl Index<MatrixIndex> for Matrix {
    type Output = f64;

    fn index(&self, i: MatrixIndex) -> &Self::Output {
        let idx = i.0 * self.columns + i.1;
        &self.elements[idx]
    }
}

impl IndexMut<MatrixIndex> for Matrix {
    fn index_mut<'a>(&'a mut self, i: (usize, usize)) -> &'a mut Self::Output {
        let idx = i.0 * self.columns + i.1;
        &mut self.elements[idx]
    }
}
