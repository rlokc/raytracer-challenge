use std::fmt::Debug;

use crate::{utils::f32_eq, tuple::Tuple};

#[derive(Debug, Clone)]
pub struct Matrix {
    pub vals: Vec<Vec<f32>>
}

fn debug_print<T: Debug>(val: T) -> T {
    println!("{:?}", val);
    val
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}

impl Matrix {
    pub fn new_from_string(vals: &str) -> Matrix {
        /*
        Constructs a new map from a string input like: 
        let s = "
        | 1    | 2    | 3    | 4    |
        | 5.5  | 6.5  | 7.5  | 8.5  |
        | 9    | 10   | 11   | 12   |
        | 13.5 | 14.5 | 15.5 | 16.5 |
        ";
        let m = Matrix::new_from_string(s);
         */

        let values = vals
        .replace('|', " ")
        .split('\n')
        .map(|line| line
            .split_whitespace()
            .filter(|s| s != &"" && s != &"|")
            .map(|val| val.parse::<f32>().unwrap())
            .collect::<Vec<f32>>()
        )
        .filter(|line| !line.is_empty())
        .collect::<Vec<Vec<f32>>>();


        Matrix { vals: values }

    }

    pub fn identity_matrix(dimensions: usize) -> Matrix {
        let mut vals = vec![vec![0.0; dimensions]; dimensions];
        for i in 0..dimensions {
                vals[i][i] = 1.0;
        }
        Matrix { vals }
    }

    pub fn new_from_tuple(tuple: &Tuple) -> Matrix {
        let vals = vec![
            vec![tuple.x],
            vec![tuple.y],
            vec![tuple.z],
            vec![tuple.w],
        ];
        Matrix { vals }
    }

    pub fn to_tuple(&self) -> Tuple {
        /* Works only if the matrix is size [4,1] */
        assert_eq!(self.n_rows(), 4);
        assert_eq!(self.n_columns(), 1);

        Tuple::new(self.get(0, 0), self.get(1, 0), self.get(2, 0), self.get(3, 0))
    }

    pub fn get(&self, i: usize, j: usize) -> f32 {
        self.vals[i][j]
    }

    pub fn n_rows(&self) -> usize {
        self.vals.len()
    }

    pub fn n_columns(&self) -> usize {
        self.vals[0].len()
    }

    pub fn equals(&self, other: &Matrix) -> bool {
        let self_iter = self.vals.iter().flatten();
        let other_iter = other.vals.iter().flatten();

        let mut zipped = self_iter.zip(other_iter);
        zipped.all(|pair| f32_eq(*pair.0, *pair.1))
    }

    pub fn mat_mul(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.n_columns(), other.n_rows());
        let mut res = vec![vec![0.0; other.n_columns()]; self.n_rows()];

        for i in 0..self.n_rows() {
            for j in 0..other.n_columns() {
                let mut acc = 0.0 as f32;
                for k in 0..self.n_columns() {
                    acc += self.get(i, k) * other.get(k, j);
                }
                res[i][j] = acc;
            }
        }
        Matrix { vals: res }
    }

    pub fn tuple_mul(&self, other: &Tuple) -> Tuple {
        self.mat_mul(&Matrix::new_from_tuple(other)).to_tuple()
    }

    pub fn transpose(&self) -> Matrix {
        let mut vals = vec![
            vec![0.0; self.n_rows()]; self.n_columns()
        ];
        for i in 0..self.n_rows() {
            for j in 0..self.n_columns() {
                vals[j][i] = self.vals[i][j];
            }
        }
        Matrix { vals }
    }

    pub fn determinant(&self) -> f32 {
        if self.n_rows() == 2 {
            return self.vals[0][0] * self.vals[1][1] - self.vals[0][1] * self.vals[1][0];
        } else {
            let mut acc = 0.0 as f32;
            for j in 0..self.n_columns() {
                acc += self.get(0, j) * self.cofactor(0, j);
            }
            return acc;
        }
    }

    pub fn submatrix(&self, row_to_remove: usize, col_to_remove: usize) -> Matrix {
        let mut vals = vec![vec![0.0; self.n_columns() - 1]; self.n_rows() - 1];
        for i in 0..self.n_rows() {
            if i == row_to_remove {
                continue;
            };
            for j in 0..self.n_columns() {
                if j == col_to_remove { 
                    continue;
                }
                let res_i = if i > row_to_remove { i - 1 } else { i };
                let res_j = if j > col_to_remove { j - 1 } else { j };
                vals[res_i][res_j] = self.get(i, j);
            }
        }
        Matrix { vals }
    }

    pub fn minor(&self, row: usize, column: usize) -> f32 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f32 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 { minor } else { -minor }
    }

    pub fn invert(&self) -> Option<Matrix> {
        if f32_eq(self.determinant(), 0.0) {
            return None
        }

        let det = self.determinant();
        let mut vals = vec![vec![0.0; self.n_columns()]; self.n_rows()];
        for i in 0..self.n_rows() {
            for j in 0..self.n_columns() {
                let val = self.cofactor(i, j);
                // Transpose along the way!!!
                vals[j][i] = val / det;
            }
        }
        return Some(Matrix { vals });
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Matrix {
        let mut res = Matrix::identity_matrix(self.n_rows());
        res.vals[0][3] = x;
        res.vals[1][3] = y;
        res.vals[2][3] = z;

        res.mat_mul(&self)
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Matrix {
        let mut res = Matrix::identity_matrix(self.n_rows());

        res.vals[0][0] = x;
        res.vals[1][1] = y;
        res.vals[2][2] = z;

        res.mat_mul(&self)
    }

    pub fn rotate_x(&self, rad: f32) -> Matrix {
        let mut res = Matrix::identity_matrix(self.n_rows());

        res.vals[1][1] = rad.cos();
        res.vals[1][2] = -rad.sin();
        res.vals[2][1] = rad.sin();
        res.vals[2][2] = rad.cos();

        res.mat_mul(&self)
    }

    pub fn rotate_y(&self, rad: f32) -> Matrix {
        let mut res = Matrix::identity_matrix(self.n_rows());

        res.vals[0][0] = rad.cos();
        res.vals[0][2] = rad.sin();
        res.vals[2][0] = -rad.sin();
        res.vals[2][2] = rad.cos();

        res.mat_mul(&self)
    }

    pub fn rotate_z(&self, rad: f32) -> Matrix {
        let mut res = Matrix::identity_matrix(self.n_rows());

        res.vals[0][0] = rad.cos();
        res.vals[0][1] = -rad.sin();
        res.vals[1][0] = rad.sin();
        res.vals[1][1] = rad.cos();

        res.mat_mul(&self)
    }

    pub fn shear(&self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix {
        let mut res = Matrix::identity_matrix(self.n_rows());

        res.vals[0][1] = xy;
        res.vals[0][2] = xz;
        res.vals[1][0] = yx;
        res.vals[1][2] = yz;
        res.vals[2][0] = zx;
        res.vals[2][1] = zy;

        self.mat_mul(&res)
    }

}