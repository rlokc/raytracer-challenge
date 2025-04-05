#[cfg(test)]
mod normal_tests {
    use raytracer::sphere::sphere;
    use raytracer::transformations::{rotate_z, translate};
    use raytracer::tuple::Tuple;
    use std::f32::consts::PI;

    #[test]
    fn normal_sphere_x() {
        let s = sphere();
        let point = Tuple::point(1.0, 0.0, 0.0);
        let expected = Tuple::vector(1.0, 0.0, 0.0);
        let actual = s.lock().unwrap().normal_at(point);

        assert_eq!(actual, expected);
    }

    #[test]
    fn normal_sphere_y() {
        let s = sphere();
        let point = Tuple::point(0.0, 1.0, 0.0);
        let expected = Tuple::vector(0.0, 1.0, 0.0);
        let actual = s.lock().unwrap().normal_at(point);

        assert_eq!(actual, expected);
    }

    #[test]
    fn normal_sphere_z() {
        let s = sphere();
        let point = Tuple::point(0.0, 0.0, 1.0);
        let expected = Tuple::vector(0.0, 0.0, 1.0);
        let actual = s.lock().unwrap().normal_at(point);

        assert_eq!(actual, expected);
    }

    #[test]
    fn normal_sphere_non_axial() {
        let s = sphere();
        let sqr = 3.0_f32.sqrt() / 3.0;
        let point = Tuple::point(sqr, sqr, sqr);
        let expected = Tuple::vector(sqr, sqr, sqr);
        let actual = s.lock().unwrap().normal_at(point);

        assert_eq!(actual, expected);
    }

    #[test]
    fn normal_sphere_is_normalized() {
        let s = sphere();
        let sqr = 3.0_f32.sqrt() / 3.0;
        let point = Tuple::point(sqr, sqr, sqr);
        let actual = s.lock().unwrap().normal_at(point);

        assert_eq!(actual, actual.normalize());
    }

    #[test]
    fn normal_sphere_translated() {
        let s = sphere();
        let transform = translate(0.0, 1.0, 0.0);
        s.lock().unwrap().set_transformation(&transform);

        let point = Tuple::point(0.0, 1.70711, -std::f32::consts::FRAC_1_SQRT_2);
        let expected = Tuple::vector(
            0.0,
            std::f32::consts::FRAC_1_SQRT_2,
            -std::f32::consts::FRAC_1_SQRT_2,
        );

        let actual = s.lock().unwrap().normal_at(point);
        assert_eq!(actual, expected);
    }

    #[test]
    fn normal_sphere_double_transform() {
        let s = sphere();
        let transform = rotate_z(PI / 5.0).scale(1.0, 0.5, 1.0);
        s.lock().unwrap().set_transformation(&transform);

        let point = Tuple::point(0.0, 2.0f32.sqrt() / 2.0, -2.0f32.sqrt() / 2.0);
        let expected = Tuple::vector(0.0, 0.97014, -0.24254);

        let actual = s.lock().unwrap().normal_at(point);
        assert_eq!(actual, expected);
    }

    #[test]
    fn vector_reflection_45() {
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let normal = Tuple::vector(0.0, 1.0, 0.0);

        let expected = Tuple::vector(1.0, 1.0, 0.0);
        assert_eq!(v.reflect(normal), expected);
    }

    #[test]
    fn vector_reflection_slanted_surface() {
        let v = Tuple::vector(0.0, -1.0, 0.0);
        let sqr = 2.0f32.sqrt() / 2.0;
        let normal = Tuple::vector(sqr, sqr, 0.0);

        let expected = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.reflect(normal), expected);
    }
}
