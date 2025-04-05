#[cfg(test)]
mod transformation_tests {
    use std::f32::consts::PI;

    use raytracer::transformations::{scale, translate, view_transform};
    use raytracer::{matrix::Matrix, tuple::Tuple};

    #[test]
    pub fn translation_test() {
        let transform = Matrix::identity_matrix(4).translate(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        let expected = Tuple::point(2.0, 1.0, 7.0);
        let actual = transform.tuple_mul(&p);

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn translation_inverse_test() {
        let transform = Matrix::identity_matrix(4).translate(5.0, -3.0, 2.0);
        let inv = transform.invert().unwrap();
        let p = Tuple::point(-3.0, 4.0, 5.0);

        let expected = Tuple::point(-8.0, 7.0, 3.0);
        let actual = inv.tuple_mul(&p);

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn translation_dont_affect_vectors() {
        let transform = Matrix::identity_matrix(4).translate(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(transform.tuple_mul(&v), v);
    }

    #[test]
    pub fn scaling_point() {
        let transform = Matrix::identity_matrix(4).scale(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        let expected = Tuple::point(-8.0, 18.0, 32.0);

        assert_eq!(transform.tuple_mul(&p), expected);
    }

    #[test]
    pub fn scaling_vector() {
        let transform = Matrix::identity_matrix(4).scale(2.0, 3.0, 4.0);
        let p = Tuple::vector(-4.0, 6.0, 8.0);

        let expected = Tuple::vector(-8.0, 18.0, 32.0);

        assert_eq!(transform.tuple_mul(&p), expected);
    }

    #[test]
    pub fn scaling_inverse() {
        let transform = Matrix::identity_matrix(4)
            .scale(2.0, 3.0, 4.0)
            .invert()
            .unwrap();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        let expected = Tuple::vector(-2.0, 2.0, 2.0);

        assert_eq!(transform.tuple_mul(&v), expected);
    }

    #[test]
    pub fn scaling_reflection() {
        let transform = Matrix::identity_matrix(4).scale(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        let expected = Tuple::point(-2.0, 3.0, 4.0);

        assert_eq!(transform.tuple_mul(&p), expected);
    }

    #[test]
    pub fn rotate_x() {
        let p = Tuple::point(0.0, 1.0, 0.0);

        let half_quarter = Matrix::identity_matrix(4).rotate_x(PI / 4.0);
        let full_quarter = Matrix::identity_matrix(4).rotate_x(PI / 2.0);

        let expected_half = Tuple::point(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0);
        let expected_full = Tuple::point(0.0, 0.0, 1.0);

        assert_eq!(half_quarter.tuple_mul(&p), expected_half);
        assert_eq!(full_quarter.tuple_mul(&p), expected_full);
    }

    #[test]
    pub fn rotate_x_inverse() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::identity_matrix(4).rotate_x(PI / 4.0);
        let inv = half_quarter.invert().unwrap();

        let expected = Tuple::point(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        assert_eq!(inv.tuple_mul(&p), expected);
    }

    #[test]
    pub fn rotate_y() {
        let p = Tuple::point(0.0, 0.0, 1.0);

        let half_quarter = Matrix::identity_matrix(4).rotate_y(PI / 4.0);
        let full_quarter = Matrix::identity_matrix(4).rotate_y(PI / 2.0);

        let expected_half = Tuple::point(2.0_f32.sqrt() / 2.0, 0.0, 2.0_f32.sqrt() / 2.0);
        let expected_full = Tuple::point(1.0, 0.0, 0.0);

        assert_eq!(half_quarter.tuple_mul(&p), expected_half);
        assert_eq!(full_quarter.tuple_mul(&p), expected_full);
    }

    #[test]
    pub fn rotate_z() {
        let p = Tuple::point(0.0, 1.0, 0.0);

        let half_quarter = Matrix::identity_matrix(4).rotate_z(PI / 4.0);
        let full_quarter = Matrix::identity_matrix(4).rotate_z(PI / 2.0);

        let expected_half = Tuple::point(-2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0, 0.0);
        let expected_full = Tuple::point(-1.0, 0.0, 0.0);

        assert_eq!(half_quarter.tuple_mul(&p), expected_half);
        assert_eq!(full_quarter.tuple_mul(&p), expected_full);
    }

    #[test]
    pub fn shear_xy() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let transform = Matrix::identity_matrix(4).shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

        let expected = Tuple::point(5.0, 3.0, 4.0);
        assert_eq!(transform.tuple_mul(&p), expected);
    }

    #[test]
    pub fn shear_xz() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let transform = Matrix::identity_matrix(4).shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);

        let expected = Tuple::point(6.0, 3.0, 4.0);
        assert_eq!(transform.tuple_mul(&p), expected);
    }

    #[test]
    pub fn shear_yx() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let transform = Matrix::identity_matrix(4).shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

        let expected = Tuple::point(2.0, 5.0, 4.0);
        assert_eq!(transform.tuple_mul(&p), expected);
    }

    #[test]
    pub fn shear_yz() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let transform = Matrix::identity_matrix(4).shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        let expected = Tuple::point(2.0, 7.0, 4.0);
        assert_eq!(transform.tuple_mul(&p), expected);
    }

    #[test]
    pub fn shear_zx() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let transform = Matrix::identity_matrix(4).shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

        let expected = Tuple::point(2.0, 3.0, 6.0);
        assert_eq!(transform.tuple_mul(&p), expected);
    }

    #[test]
    pub fn shear_zy() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let transform = Matrix::identity_matrix(4).shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        let expected = Tuple::point(2.0, 3.0, 7.0);
        assert_eq!(transform.tuple_mul(&p), expected);
    }

    #[test]
    pub fn transformation_chaining() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix::identity_matrix(4).rotate_x(PI / 2.0);
        let b = Matrix::identity_matrix(4).scale(5.0, 5.0, 5.0);
        let c = Matrix::identity_matrix(4).translate(10.0, 5.0, 7.0);

        let p2 = a.tuple_mul(&p);
        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));

        let p3 = b.tuple_mul(&p2);
        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));

        let p4 = c.tuple_mul(&p3);
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));

        let t = c.mat_mul(&b).mat_mul(&a);
        let p5 = t.tuple_mul(&p);
        assert_eq!(p4, p5);

        let t_var2 = Matrix::identity_matrix(4)
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        assert_eq!(t_var2.tuple_mul(&p), p4);
    }

    #[test]
    pub fn view_transform_default() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, -1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let actual = view_transform(from, to, up);
        let expected = Matrix::identity_matrix(4);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn view_transform_positive_z() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, 1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let actual = view_transform(from, to, up);
        let expected = scale(-1.0, 1.0, -1.0);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn view_transform_move_world() {
        let from = Tuple::point(0.0, 0.0, 8.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let actual = view_transform(from, to, up);
        let expected = translate(0.0, 0.0, -8.0);

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn view_transform_arbitrary() {
        let from = Tuple::point(1.0, 3.0, 2.0);
        let to = Tuple::point(4.0, -2.0, 8.0);
        let up = Tuple::vector(1.0, 1.0, 0.0);

        let actual = view_transform(from, to, up);
        let expected = Matrix::new_from_string(
            "
| -0.50709 | 0.50709 | 0.67612 | -2.36643 |
| 0.76772 | 0.60609 | 0.12122 | -2.82843 |
| -0.35857 | 0.59761 | -0.71714 | 0.00000 |
| 0.00000 | 0.00000 | 0.00000 | 1.00000 |
        ",
        );

        assert_eq!(actual, expected);
    }
}
