#[cfg(test)]
mod camera_tests {
    use raytracer::camera::Camera;
    use raytracer::matrix::Matrix;
    use raytracer::transformations::translate;
    use raytracer::tuple::Tuple;
    use std::f64::consts::PI;
    use raytracer::utils::f64_eq;

    #[test]
    pub fn default_camera() {
        let hsize = 160;
        let vsize = 120;
        let fov = PI / 2.0;

        let c = Camera::new(hsize, vsize, fov);

        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert_eq!(c.fov, fov);
        assert_eq!(c.transform, Matrix::identity_matrix(4));
    }

    #[test]
    pub fn pixel_size_horizontal() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(f64_eq(c.pixel_size, 0.01));
    }

    #[test]
    pub fn pixel_size_vertical() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(f64_eq(c.pixel_size, 0.01));
    }

    #[test]
    pub fn ray_through_center() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    pub fn ray_through_corner() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    pub fn ray_through_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = translate(0.0, -2.0, 5.0).rotate_y(PI / 4.0);

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0.0, 2.0, -5.0));
        let sqrt2 = 2.0_f64.sqrt() / 2.0;
        assert_eq!(r.direction, Tuple::vector(sqrt2, 0.0, -sqrt2));
    }
}
