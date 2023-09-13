#[cfg(test)]
mod ray_tests {
    use std::sync::{Arc, Mutex};

    use raytracer::{tuple::Tuple, ray::Ray, sphere::sphere, intersection::{Intersection, Intersections}, matrix::Matrix};

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

        let res = ray.intersect(s.clone());

        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(4.0, s.clone()));
        assert_eq!(res[1], Intersection::new(6.0, s.clone()));
    }

    #[test]
    fn intersection_one_point() {
        let ray = Ray::new_flat(0.0, 1.0, -5.0, 0.0, 0.0, 1.0);
        let s = sphere();

        let res = ray.intersect(s.clone());

        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(5.0, s.clone()));
        assert_eq!(res[1], Intersection::new(5.0, s.clone()));
    }

    #[test]
    fn intersection_no_point() {
        let ray = Ray::new_flat(0.0, 2.0, -5.0, 0.0, 0.0, 1.0);
        let s = sphere();

        let res = ray.intersect(s.clone());
        assert_eq!(res.len(), 0)
    }

    #[test]
    fn intersection_inside() {
        let ray = Ray::new_flat(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let s = sphere();

        let res = ray.intersect(s.clone());

        assert_eq!(res[0], Intersection::new(-1.0, s.clone()));
        assert_eq!(res[1], Intersection::new(1.0, s.clone()));
    }

    #[test]
    fn intersection_ahead() {
        let ray = Ray::new_flat(0.0, 0.0, 5.0, 0.0, 0.0, 1.0);
        let s = sphere();

        let res = ray.intersect(s.clone());
        
        assert_eq!(res[0], Intersection::new(-6.0, s.clone()));
        assert_eq!(res[1], Intersection::new(-4.0, s.clone()));
    }

    #[test]
    fn intersections_aggregation() {
        let s = sphere();

        let i1 = Arc::new(Intersection::new(1.0, s.clone()));
        let i2 = Arc::new(Intersection::new(2.0, s.clone()));

        let mut xs = Intersections::new();
        xs.push(i1);
        xs.push(i2);

        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn intersections_keeps_object() {
        let s = sphere();

        let r = Ray::new_flat(0.0, 0.0, -5.0, 0.0, 0.0, 1.0);

        let xs = r.intersect(s.clone());
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].scene_object.id(), s.id());
        assert_eq!(xs[1].scene_object.id(), s.id());
    }

    #[test]
    fn intersections_hit_1() {
        let s = sphere();

        let i1 = Arc::new(Intersection::new(1.0, s.clone()));
        let i2 = Arc::new(Intersection::new(2.0, s.clone()));

        let mut xs = Intersections::new();
        xs.push(i1.clone()).push(i2.clone());

        let i = xs.hit().unwrap();
        assert_eq!(i, i1);
    }

    #[test]
    fn intersections_hit_2() {
        let s = sphere();

        let i1 = Arc::new(Intersection::new(-1.0, s.clone()));
        let i2 = Arc::new(Intersection::new(1.0, s.clone()));

        let mut xs = Intersections::new();
        xs.push(i1.clone()).push(i2.clone());

        let i = xs.hit().unwrap();
        assert_eq!(i, i2);
    }


    #[test]
    fn intersections_hit_3() {
        let s = sphere();

        let i1 = Arc::new(Intersection::new(-2.0, s.clone()));
        let i2 = Arc::new(Intersection::new(-1.0, s.clone()));

        let mut xs = Intersections::new();
        xs.push(i1.clone()).push(i2.clone());

        let i = xs.hit();
        assert!(i.is_none());
    }

    #[test]
    fn intersections_hit_first_nonnegative() {
        let s = sphere();

        let i1 = Arc::new(Intersection::new(5.0, s.clone()));
        let i2 = Arc::new(Intersection::new(7.0, s.clone()));
        let i3 = Arc::new(Intersection::new(-3.0, s.clone()));
        let i4 = Arc::new(Intersection::new(2.0, s.clone()));

        let mut xs = Intersections::new();
        xs.push(i1.clone()).push(i2.clone()).push(i3.clone()).push(i4.clone());

        let i = xs.hit().unwrap();
        assert_eq!(i, i4);
    }

    #[test]
    fn ray_translation() {
        let r = Ray::new_flat(1.0, 2.0, 3.0, 0.0, 1.0, 0.0);
        let m = Matrix::identity_matrix(4).translate(3.0, 4.0, 5.0);

        let r2 = r.transform(&m);

        assert_eq!(r2.origin, Tuple::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn ray_scaling() {
        let r = Ray::new_flat(1.0, 2.0, 3.0, 0.0, 1.0, 0.0);
        let m = Matrix::identity_matrix(4).scale(2.0, 3.0, 4.0);

        let r2 = r.transform(&m);

        assert_eq!(r2.origin, Tuple::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 3.0, 0.0));
    }

    #[test]
    fn sphere_default() {
        let s = sphere();

        assert!(s.transformation().get_mut().unwrap().equals(&Matrix::identity_matrix(4)));
    }

    #[test]
    fn sphere_transformation() {
        let mut s = sphere();

        let t = Matrix::identity_matrix(4).translate(2.0, 3.0, 4.0);

        s.set_transformation(Arc::new(Mutex::new(t)));
    }




}