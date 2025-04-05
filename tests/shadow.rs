#[cfg(test)]
mod shadow_tests {
    use raytracer::colors::Color;
    use raytracer::intersection::{is_shadowed, prepare_computations, shade_hit, Intersection};
    use raytracer::light::PointLight;
    use raytracer::ray::Ray;
    use raytracer::sphere::{sphere, Sphere};
    use raytracer::transformations::translate;
    use raytracer::tuple::Tuple;
    use raytracer::utils::EPSILON;
    use raytracer::world::World;
    use std::sync::Arc;

    #[test]
    pub fn no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default_world();
        let p = Tuple::point(0.0, 10.0, 0.0);
        assert!(!is_shadowed(Arc::new(w), p));
    }

    #[test]
    pub fn shadow_when_object_between_point_and_light() {
        let w = World::default_world();
        let p = Tuple::point(10.0, -10.0, 10.0);
        assert!(is_shadowed(Arc::new(w), p));
    }

    #[test]
    pub fn no_shadow_when_object_behind_light() {
        let w = World::default_world();
        let p = Tuple::point(-20.0, 20.0, -20.0);
        assert!(!is_shadowed(Arc::new(w), p));
    }

    #[test]
    pub fn no_shadow_whe_object_behind_point() {
        let w = World::default_world();
        let p = Tuple::point(-2.0, 2.0, -2.0);
        assert!(!is_shadowed(Arc::new(w), p));
    }

    #[test]
    pub fn shade_hit_with_intersection_in_shadow() {
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let mut world = World::new();
        let sphere1 = sphere();
        world.objects.push(sphere1);

        let sphere2 = sphere();
        let sphere2_transformation = translate(0.0, 0.0, 10.0);
        sphere2
            .lock()
            .unwrap()
            .set_transformation(&sphere2_transformation);
        world.objects.push(sphere2.clone());

        world.light_sources.push(light);

        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, sphere2);

        let comps = prepare_computations(Arc::new(i), r);
        let c = shade_hit(Arc::new(world), &comps);

        let expected_color = Color::new(0.1, 0.1, 0.1);
        assert_eq!(c, expected_color);
    }

    #[test]
    pub fn hit_point_offset() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = sphere();

        let s_transform = translate(0.0, 0.0, 1.0);
        s.lock().unwrap().set_transformation(&s_transform);

        let i = Intersection::new(5.0, s);
        let comps = prepare_computations(Arc::new(i), r);

        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
