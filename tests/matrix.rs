#[cfg(test)]
mod matrix_tests {
    use raytracer::{matrix::Matrix, tuple::Tuple, utils::f32_eq};

    #[test]
    pub fn matrix_creation() {
        let s = "
        | 1    | 2    | 3    | 4    |
        | 5.5  | 6.5  | 7.5  | 8.5  |
        | 9    | 10   | 11   | 12   |
        | 13.5 | 14.5 | 15.5 | 16.5 |
        ";
        let m = Matrix::new_from_string(s);

        assert!(f32_eq(m.get(0, 0), 1.0));
        assert!(f32_eq(m.get(0, 3), 4.0));
        assert!(f32_eq(m.get(1, 0), 5.5));
        assert!(f32_eq(m.get(1, 2), 7.5));
        assert!(f32_eq(m.get(2, 2), 11.0));
        assert!(f32_eq(m.get(3, 0), 13.5));
        assert!(f32_eq(m.get(3, 2), 15.5));
    }

    #[test]
    pub fn matrix_2x2() {
        let s = "|-3|5|
        |1|-2|";

        let m = Matrix::new_from_string(s);

        assert!(f32_eq(m.get(0, 0), -3.0));
        assert!(f32_eq(m.get(0, 1), 5.0));
        assert!(f32_eq(m.get(1, 0), 1.0));
        assert!(f32_eq(m.get(1, 1), -2.0));
    }

    #[test]
    pub fn matrix_3x3() {
        let s = "|-3|5|0|
        |1|-2|-7|
        |0|1|1|";

        let m = Matrix::new_from_string(s);

        assert!(f32_eq(m.get(0, 0), -3.0));
        assert!(f32_eq(m.get(1, 1), -2.0));
        assert!(f32_eq(m.get(2, 2), 1.0));
    }

    #[test]
    pub fn matrix_equality() {
        let s = "
        | 1 | 2 | 3 | 4 |
        | 5 | 6 | 7 | 8 |
        | 9 | 8 | 7 | 6 |
        | 5 | 4 | 3 | 2 |
        ";

        let m1 = Matrix::new_from_string(s);
        let m2 = Matrix::new_from_string(s);
        assert_eq!(m1, m2);
    }

    #[test]
    pub fn matrix_inequality() {
        let s1 = "
        | 1 | 2 | 3 | 4 |
        | 5 | 6 | 7 | 8 |
        | 9 | 8 | 7 | 6 |
        | 5 | 4 | 3 | 2 |
        ";

        let s2 = "
        | 2 | 3 | 4 | 5 |
        | 6 | 7 | 8 | 9 |
        | 8 | 7 | 6 | 5 |
        | 4 | 3 | 2 | 1 |
        ";

        let m1 = Matrix::new_from_string(s1);
        let m2 = Matrix::new_from_string(s2);

        assert_ne!(m1, m2);
    }

    #[test]
    pub fn matrix_multiplication() {
        let s1 = "
        | 1 | 2 | 3 | 4 |
        | 5 | 6 | 7 | 8 |
        | 9 | 8 | 7 | 6 |
        | 5 | 4 | 3 | 2 |
        ";

        let s2 = "
        | -2 | 1 | 2 | 3 |
        | 3 | 2 | 1 | -1 |
        | 4 | 3 | 6 | 5 |
        | 1 | 2 | 7 | 8 |
        ";

        let smul = "
        | 20| 22 | 50 | 48 |
        | 44| 54 | 114 | 108 |
        | 40| 58 | 110 | 102 |
        | 16| 26 | 46 | 42 |
        ";

        let m1 = Matrix::new_from_string(s1);
        let m2 = Matrix::new_from_string(s2);
        let expected = Matrix::new_from_string(smul);
        let actual = m1.mat_mul(&m2);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn tuple_multiplication() {
        let s = "
        | 1 | 2 | 3 | 4 |
        | 2 | 4 | 4 | 2 |
        | 8 | 6 | 4 | 1 |
        | 0 | 0 | 0 | 1 |
        ";

        let m = Matrix::new_from_string(s);
        let t = Tuple::new(1.0, 2.0, 3.0, 1.0);

        let expected = Tuple::new(18.0, 24.0, 33.0, 1.0);
        let actual = m.tuple_mul(&t);

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn multiply_identity() {
        let s = "
        | 0 | 1 | 2 | 4 |
        | 1 | 2 | 4 | 8 |
        | 2 | 4 | 8 | 16 |
        | 4 | 8 | 16 | 32 |
        ";

        let m = Matrix::new_from_string(s);
        let id = Matrix::identity_matrix(4);

        assert_eq!(m.mat_mul(&id), m);
    }

    #[test]
    pub fn transpose() {
        let s = "
        | 0 | 9 | 3 | 0 |
        | 9 | 8 | 0 | 8 |
        | 1 | 8 | 5 | 3 |
        | 0 | 0 | 5 | 8 |
        ";

        let transposed = "
        | 0 | 9 | 1 | 0 |
        | 9 | 8 | 8 | 0 |
        | 3 | 0 | 5 | 5 |
        | 0 | 8 | 3 | 8 |
        ";

        let m = Matrix::new_from_string(s);
        let expected = Matrix::new_from_string(transposed);
        let actual = m.transpose();
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn transpose_identity() {
        let id = Matrix::identity_matrix(4);
        let transposed = id.transpose();
        assert_eq!(transposed, id);
    }

    #[test]
    pub fn determinant() {
        let s = "
        | 1 | 5 |
        | -3 | 2 |
        ";

        let m = Matrix::new_from_string(s);
        let actual = m.determinant();
        assert!(f32_eq(actual, 17.0));
    }

    #[test]
    pub fn submatrix_3x3() {
        let s = "
        | 1 | 5 | 0 |
        | -3 | 2 | 7 |
        | 0 | 6 | -3 |
        ";

        let expected = "
        | -3 | 2 |
        | 0 | 6 |
        ";

        let m = Matrix::new_from_string(s);
        let expected = Matrix::new_from_string(expected);

        let actual = m.submatrix(0, 2);
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn submatrix_4x4() {
        let s = "
        | -6 | 1 | 1 | 6 |
        | -8 | 5 | 8 | 6 |
        | -1 | 0 | 8 | 2 |
        | -7 | 1 | -1 | 1 |
        ";

        let expected = "
        | -6 | 1 | 6 |
        | -8 | 8 | 6 |
        | -7 | -1 | 1 |
        ";

        let m = Matrix::new_from_string(s);
        let expected = Matrix::new_from_string(expected);

        let actual = m.submatrix(2, 1);
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn minor() {
        let s = "
        | 3 | 5 | 0 |
        | 2 | -1 | -7 |
        | 6 | -1 | 5 |
        ";

        let m = Matrix::new_from_string(s);
        let b = m.submatrix(1, 0);

        assert_eq!(b.determinant(), 25.0);
        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    pub fn cofactor() {
        let s = "
        | 3 | 5 | 0 |
        | 2 | -1 | -7 |
        | 6 | -1 | 5 |
        ";
        let m = Matrix::new_from_string(s);

        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);

        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    pub fn determinant_3x3() {
        let s = "
        | 1 | 2 | 6 |
        | -5 | 8 | -4 |
        | 2 | 6 | 4 |
        ";

        let m = Matrix::new_from_string(s);

        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);

        assert_eq!(m.determinant(), -196.0);
    }

    #[test]
    pub fn determinant_4x4() {
        let s = "
        | -2 | -8 | 3 | 5 |
        | -3 | 1 | 7 | 3 |
        | 1 | 2 | -9 | 6 |
        | -6 | 7 | 7 | -9 |
        ";

        let m = Matrix::new_from_string(s);

        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);

        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    pub fn matrix_inversibility() {
        let s = "
        | 6 | 4 | 4 | 4 |
        | 5 | 5 | 7 | 6 |
        | 4 | -9 | 3 | -7 |
        | 9 | 1 | 7 | -6 |
        ";

        let m = Matrix::new_from_string(s);
        assert_eq!(m.determinant(), -2120.0);
        assert!(m.invert().is_some());

        let s2 = "
        | -4 | 2 | -2 | -3 |
        | 9 | 6 | 2 | 6 |
        | 0 | -5 | 1 | -5 |
        | 0 | 0 | 0 | 0 |
        ";

        let m2 = Matrix::new_from_string(s2);
        assert_eq!(m2.determinant(), 0.0);
        assert!(m2.invert().is_none());
    }

    #[test]
    pub fn matrix_inversion() {
        let s = "
        | -5 | 2 | 6 | -8 |
        | 1 | -5 | 1 | 8 |
        | 7 | 7 | -6 | -7 |
        | 1 | -3 | 7 | 4 |
        ";

        let expected = "
        | 0.21805 | 0.45113 | 0.24060 | -0.04511 |
        | -0.80827 | -1.45677 | -0.44361 | 0.52068 |
        | -0.07895 | -0.22368 | -0.05263 | 0.19737 |
        | -0.52256 | -0.81391 | -0.30075 | 0.30639 |
        ";

        let m = Matrix::new_from_string(s);
        let expected = Matrix::new_from_string(expected);
        let actual = m.invert().unwrap();

        assert_eq!(m.determinant(), 532.0);
        assert_eq!(m.cofactor(2, 3), -160.0);
        assert_eq!(actual.get(3, 2), -160.0 / 532.0);

        assert_eq!(m.cofactor(3, 2), 105.0);
        assert_eq!(actual.get(2, 3), 105.0 / 532.0);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn matrix_inversion_2() {
        let s = "
        | 8 | -5 | 9 | 2 |
        | 7 | 5 | 6 | 1 |
        | -6 | 0 | 9 | 6 |
        | -3 | 0 | -9 | -4 |
        ";

        let expected = "
        | -0.15385 | -0.15385 | -0.28205 | -0.53846 |
        | -0.07692 | 0.12308 | 0.02564 | 0.03077 |
        | 0.35897 | 0.35897 | 0.43590 | 0.92308 |
        | -0.69231 | -0.69231 | -0.76923 | -1.92308 |
        ";

        let m = Matrix::new_from_string(s);
        let expected = Matrix::new_from_string(expected);

        assert_eq!(m.invert().unwrap(), expected);
    }

    #[test]
    pub fn matrix_inversion_3() {
        let s = "
        | 9 | 3 | 0 | 9 |
        | -5 | -2 | -6 | -3 |
        | -4 | 9 | 6 | 4 |
        | -7 | 6 | 6 | 2 |
        ";

        let expected = "
        | -0.04074 | -0.07778 | 0.14444 | -0.22222 |
        | -0.07778 | 0.03333 | 0.36667 | -0.33333 |
        | -0.02901 | -0.14630 | -0.10926 | 0.12963 |
        | 0.17778 | 0.06667 | -0.26667 | 0.33333 |
        ";

        let m = Matrix::new_from_string(s);
        let expected = Matrix::new_from_string(expected);

        assert_eq!(m.invert().unwrap(), expected);
    }

    #[test]
    pub fn inversion_multiplication() {
        let a = "
        | 3 | -9 | 7 | 3 |
        | 3 | -8 | 2 | -9 |
        | -4 | 4 | 4 | 1 |
        | -6 | 5 | -1 | 1 |
        ";

        let b = "
        | 8 | 2 | 2 | 2 |
        | 3 | -1 | 7 | 0 |
        | 7 | 0 | 5 | 4 |
        | 6 | -2 | 0 | 5 |
        ";

        let a = Matrix::new_from_string(a);
        let b = Matrix::new_from_string(b);

        let c = a.mat_mul(&b);
        assert_eq!(c.mat_mul(&b.invert().unwrap()), a);
    }
}
