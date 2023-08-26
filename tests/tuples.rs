#[cfg(test)]
mod tuple_tests {
    use raytracer::{tuple::Tuple, utils::f32_eq};

    #[test]
    fn point_test() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn vector_test() {
        let a = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn create_point() {
        let a = Tuple::point(4.0, -4.0, 3.0);
        let expected = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 1.0,
        };
        assert_eq!(a, expected);
    }

    #[test]
    fn create_vector() {
        let a = Tuple::vector(4.0, -4.0, 3.0);
        let expected = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 0.0,
        };
        assert_eq!(a, expected);
    }

    #[test]
    fn test_addition() {
        let a = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let b = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };
        let expected = Tuple {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 1.0
        };
        let actual = a.add(b);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_subtraction() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b = Tuple::point(5.0, 6.0, 7.0);
        let expected = Tuple::vector(-2.0, -4.0, -6.0);
        let actual = a.sub(b);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_subtracting_vector_from_point() {
        let point = Tuple::point(3.0, 2.0, 1.0);
        let vector = Tuple::vector(5.0, 6.0, 7.0);
        let expected = Tuple::point(-2.0, -4.0, -6.0);
        let actual = point.sub(vector);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_subtracting_vector_from_vector() {
        let vec1 = Tuple::vector(3.0, 2.0, 1.0);
        let vec2 = Tuple::vector(5.0, 6.0, 7.0);
        let expected = Tuple::vector(-2.0, -4.0, -6.0);
        let actual = vec1.sub(vec2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_negation() {
        let a = Tuple{x: 1.0, y: -2.0, z: 3.0, w: -4.0};
        let expected = Tuple{x: -1.0, y: 2.0, z: -3.0, w: 4.0};
        let actual = a.negate();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_mul() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let mul = 3.5;
        let expected = Tuple::new(3.5, -7.0, 10.5, -14.0);
        let actual = a.scalar_mul(mul);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_scalar_mul_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let mul = 0.5;
        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        let actual = a.scalar_mul(mul);
        assert_eq!(actual, expected);
    }
        
    #[test]
    fn test_scalar_div() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let actual = a.scalar_div(2.0);
        let expected = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Tuple::vector(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Tuple::vector(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(Tuple::vector(1.0, 2.0, 3.0).magnitude(), 14.0_f32.sqrt());
        assert_eq!(Tuple::vector(-1.0, -2.0, -3.0).magnitude(), 14.0_f32.sqrt());
    }

    #[test]
    fn test_normalization() {
        let v1 = Tuple::vector(4.0, 0.0, 0.0);
        let v1_normalized = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v1.normalize(), v1_normalized);

        let v2 = Tuple::vector(1.0, 2.0, 3.0);
        let sq = 14.0_f32.sqrt();
        let v2_normalized = Tuple::vector(1.0/sq, 2.0/sq, 3.0/sq);
        assert_eq!(v2.normalize(), v2_normalized);
    }

    #[test]
    fn test_normalized_magnitude() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        assert!(f32_eq(v1.normalize().magnitude(), 1.0));
    }

    #[test]
    fn test_dot_product() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert!(f32_eq(a.dot(b), 20.0))
    }

    #[test]
    fn test_cross_product() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        let cross_ab = Tuple::vector(-1.0, 2.0, -1.0);
        let cross_ba = Tuple::vector(1.0, -2.0, 1.0);

        assert_eq!(a.cross(b), cross_ab);
        assert_eq!(b.cross(a), cross_ba);
    }

}
