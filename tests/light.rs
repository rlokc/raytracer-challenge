#[cfg(test)]
mod light_tests {
    use raytracer::colors::Color;
    use raytracer::light::{lighting, PointLight};
    use raytracer::material::Material;
    use raytracer::sphere::sphere;
    use raytracer::tuple::Tuple;

    #[test]
    fn test_point_light() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);

        let light = PointLight::new(position, intensity);

        assert_eq!(light.intensity, intensity);
        assert_eq!(light.position, position);
    }

    #[test]
    fn test_sphere_has_material() {
        let s = sphere();
        let mat = s.lock().unwrap().material();
        assert_eq!(mat, Material::default())
    }

    #[test]
    fn test_sphere_can_assign_material() {
        let s = sphere();
        let mut mat = Material::default();
        mat.ambient = 1.0;

        s.lock().unwrap().set_material(&mat);
        let sphere_mat = s.lock().unwrap().material();
        assert_eq!(sphere_mat, mat);
    }

    #[test]
    fn test_lighting_eye_between_light_and_surface() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let expected = Color::new(1.9, 1.9, 1.9);
        let actual = lighting(&m, &light, position, eye, normal, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lighting_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let sqrt = 2.0f64.sqrt() / 2.0;
        let eye = Tuple::vector(0.0, sqrt, -sqrt);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let expected = Color::new(1.0, 1.0, 1.0);
        let actual = lighting(&m, &light, position, eye, normal, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lighting_eye_opposite_surface_light_offset_45() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let expected = Color::new(0.7364, 0.7364, 0.7364);
        let actual = lighting(&m, &light, position, eye, normal, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lighting_eye_on_reflection_vector() {
        let sqrt = 2.0f64.sqrt() / 2.0;
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, -sqrt, -sqrt);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let expected = Color::new(1.6364, 1.6364, 1.6364);
        let actual = lighting(&m, &light, position, eye, normal, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lighting_light_behind_surface() {
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        let expected = Color::new(0.1, 0.1, 0.1);
        let actual = lighting(&m, &light, position, eye, normal, false);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lighting_with_surface_in_shadow() {
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let m = Material::default();
        let position = Tuple::point(0.0, 0.0, 0.0);

        let expected = Color::new(0.1, 0.1, 0.1);
        let actual = lighting(&m, &light, position, eye, normal, true);

        assert_eq!(expected, actual);
    }
}
