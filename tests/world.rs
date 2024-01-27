#[cfg(test)]
mod world_tests {
    use raytracer::intersection::{intersect_world, Intersection, prepare_computations};
    use raytracer::ray::Ray;
    use raytracer::sphere::sphere;
    use raytracer::tuple::Tuple;
    use raytracer::world::World;

    #[test]
    fn test_new_world() {
        let w = World::new();
        assert_eq!(w.light_sources.len(), 0);
        assert_eq!(w.objects.len(), 0);
    }

    #[test]
    fn test_default_world() {
        let w = World::default_world();
        assert_eq!(w.light_sources.len(), 1);
        assert_eq!(w.objects.len(), 2);
    }

    #[test]
    fn test_world_intersection() {
        let w = World::default_world();
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = intersect_world(w, ray);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn test_intersection_precompute() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = Intersection::new(4.0, shape.clone());

        let actual = prepare_computations(i, r);

        let shape_id = shape.lock().unwrap().id();

        assert_eq!(actual.scene_object.lock().unwrap().id(), shape_id);
        assert_eq!(actual.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(actual.eye_vector, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(actual.normal_vector, Tuple::vector(0.0, 0.0, -1.0));
    }
}