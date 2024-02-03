#[cfg(test)]
mod world_tests {
    use std::sync::Arc;
    use raytracer::colors::Color;
    use raytracer::intersection::{color_at, intersect_world, Intersection, prepare_computations, shade_hit};
    use raytracer::light::PointLight;
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

        let xs = intersect_world(Arc::new(w), ray);

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
        let i = Arc::new(Intersection::new(4.0, shape.clone()));

        let actual = prepare_computations(i, r);

        let shape_id = shape.lock().unwrap().id();

        assert_eq!(actual.scene_object.lock().unwrap().id(), shape_id);
        assert_eq!(actual.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(actual.eye_vector, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(actual.normal_vector, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_precompute_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = Arc::new(Intersection::new(4.0, shape.clone()));

        let actual = prepare_computations(i, r);
        assert_eq!(actual.is_inside_object, false);
    }

    #[test]
    fn test_precompute_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = sphere();
        let i = Arc::new(Intersection::new(1.0, shape.clone()));

        let actual = prepare_computations(i, r);

        assert_eq!(actual.is_inside_object, true);
        assert_eq!(actual.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(actual.eye_vector, Tuple::vector(0.0, 0.0, -1.0));
        // Normal is usually (0, 0, 1), but it's inverted because is inside.
        assert_eq!(actual.normal_vector, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_intersection_shading() {
        let w = World::default_world();
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.objects[0].clone();
        let i = Arc::new(Intersection::new(4.0, shape.clone()));

        let comps = prepare_computations(i, ray);
        let color = shade_hit(Arc::new(w), &comps);

        assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_intersection_shading_inside() {
        let mut w = World::default_world();
        w.light_sources[0] = PointLight::new(Tuple::point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0));
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.objects[1].clone();
        let i = Arc::new(Intersection::new(0.5, shape.clone()));

        let comps = prepare_computations(i, ray);
        let color = shade_hit(Arc::new(w), &comps);

        assert_eq!(color, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_color_ray_miss() {
        let w = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));

        let actual = color_at(Arc::new(w), r);
        let expected = Color::new(0.0, 0.0, 0.0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_color_ray_hit() {
        let w = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let actual = color_at(Arc::new(w), r);
        let expected = Color::new(0.38066, 0.47583, 0.2855);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_color_intersection_inside_ray() {
        let w = Arc::new(World::default_world());

        let outer = &w.clone().objects[0];
        let mut mut1 = outer.lock().unwrap().material();
        mut1.ambient = 1.0;
        outer.lock().unwrap().set_material(&mut1);

        let inner = &w.clone().objects[1];
        let mut mut2 = inner.lock().unwrap().material();
        mut2.ambient = 1.0;
        inner.lock().unwrap().set_material(&mut2);

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));

        let actual = color_at(w.clone(), r);
        let expected = inner.clone().lock().unwrap().material().color;

        assert_eq!(actual, expected);
    }
}