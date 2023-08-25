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

}