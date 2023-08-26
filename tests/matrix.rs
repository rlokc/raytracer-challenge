
#[cfg(test)]
mod matrix_tests {
    use raytracer::{matrix::Matrix, utils::f32_eq, tuple::Tuple};

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
}