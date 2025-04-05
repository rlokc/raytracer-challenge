#[cfg(test)]
mod render_tests {
    use raytracer::camera::Camera;
    use raytracer::colors::Color;
    use raytracer::render::render;
    use raytracer::transformations::view_transform;
    use raytracer::tuple::Tuple;
    use raytracer::world::World;
    use std::f32::consts::PI;
    use std::sync::Arc;

    #[test]
    fn test_default_render() {
        let w = World::default_world();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        c.transform = view_transform(from, to, up);

        let image = render(Arc::new(c), Arc::new(w));
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
