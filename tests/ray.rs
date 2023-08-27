#[cfg(test)]
mod ray_tests {
    use raytracer::{tuple::Tuple, ray::Ray, sphere::sphere};

    #[test]
    fn ray_creation() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);

        let ray = Ray::new(origin, direction);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn ray_position() {
        let origin = Tuple::point(2.0, 3.0, 4.0);
        let direction = Tuple::vector(1.0, 0.0, 0.0);

        let ray = Ray::new(origin, direction);

        assert_eq!(ray.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn intersection_two_points() {
        let ray = Ray::new_flat(0.0, 0.0, -5.0, 0.0, 0.0, 1.0);
        let s = sphere();

        let res = ray.intersect(s);
        assert_eq!(res, Some((4.0, 6.0)));
    }

    #[test]
    fn intersection_one_point() {
        let ray = Ray::new_flat(0.0, 1.0, -5.0, 0.0, 0.0, 1.0);
        let s = sphere();

        let res = ray.intersect(s);
        assert_eq!(res, Some((5.0, 5.0)));
    }

    #[test]
    fn intersection_no_point() {
        let ray = Ray::new_flat(0.0, 2.0, -5.0, 0.0, 0.0, 1.0);
        let s = sphere();

        let res = ray.intersect(s);
        assert_eq!(res, None);
    }

    #[test]
    fn intersection_inside() {
        let ray = Ray::new_flat(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let s = sphere();

        let res = ray.intersect(s);
        assert_eq!(res, Some((-1.0, 1.0)));
    }

    #[test]
    fn intersection_ahead() {
        let ray = Ray::new_flat(0.0, 0.0, 5.0, 0.0, 0.0, 1.0);
        let s = sphere();

        let res = ray.intersect(s);
        assert_eq!(res, Some((-6.0, -4.0)));
    }

}