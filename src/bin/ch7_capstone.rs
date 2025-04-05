use raytracer::camera::Camera;
use raytracer::colors::Color;
use raytracer::light::PointLight;
use raytracer::material::Material;
use raytracer::render::render;
use raytracer::sphere::sphere;
use raytracer::transformations::{scale, translate, view_transform};
use raytracer::tuple::Tuple;
use raytracer::world::World;
use std::f64::consts::PI;
use std::sync::Arc;

fn generate_world() -> World {
    let mut world = World::new();

    let floor = sphere();
    let floor_transform = scale(10.0, 0.01, 10.0);
    let mut floor_material = Material::default();
    floor_material.color = Color::new(1.0, 0.9, 0.9);
    floor_material.specular = 0.0;
    let mut guard = floor.lock().unwrap();
    guard.set_transformation(&floor_transform);
    guard.set_material(&floor_material);

    world.objects.push(floor.clone());

    let left_wall = sphere();
    let left_wall_transform = scale(10.0, 0.01, 10.0)
        .rotate_x(PI / 2.0)
        .rotate_y(-PI / 4.0)
        .translate(0.0, 0.0, 5.0);
    let mut guard = left_wall.lock().unwrap();
    guard.set_material(&floor_material);
    guard.set_transformation(&left_wall_transform);
    world.objects.push(left_wall.clone());

    let right_wall = sphere();
    let right_wall_transform = scale(10.0, 0.01, 10.0)
        .rotate_x(PI / 2.0)
        .rotate_y(PI / 4.0)
        .translate(0.0, 0.0, 5.0);
    let mut guard = right_wall.lock().unwrap();
    guard.set_material(&floor_material);
    guard.set_transformation(&right_wall_transform);
    world.objects.push(right_wall.clone());

    let middle_sphere = sphere();
    let middle_sphere_transform = translate(-0.5, 1.0, 0.5);
    let mut middle_sphere_material = Material::default();
    middle_sphere_material.color = Color::new(0.1, 1.0, 0.5);
    middle_sphere_material.diffuse = 0.7;
    middle_sphere_material.specular = 0.3;
    let mut guard = middle_sphere.lock().unwrap();
    guard.set_transformation(&middle_sphere_transform);
    guard.set_material(&middle_sphere_material);
    world.objects.push(middle_sphere.clone());

    let right_sphere = sphere();
    let right_sphere_transform = scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5);
    let mut right_sphere_material = Material::default();
    right_sphere_material.color = Color::new(0.5, 1.0, 0.1);
    right_sphere_material.diffuse = 0.7;
    right_sphere_material.specular = 0.3;
    let mut guard = right_sphere.lock().unwrap();
    guard.set_transformation(&right_sphere_transform);
    guard.set_material(&middle_sphere_material);
    world.objects.push(right_sphere.clone());

    let left_sphere = sphere();
    let left_sphere_transform = scale(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75);
    let mut left_sphere_material = Material::default();
    left_sphere_material.color = Color::new(1.0, 0.8, 0.1);
    left_sphere_material.diffuse = 0.7;
    left_sphere_material.specular = 0.3;
    let mut guard = left_sphere.lock().unwrap();
    guard.set_transformation(&left_sphere_transform);
    guard.set_material(&left_sphere_material);
    world.objects.push(left_sphere.clone());

    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    world.light_sources.push(light);

    world
}

pub fn main() {
    let w = generate_world();
    let mut c = Camera::new(1000, 500, PI / 3.0);
    let from = Tuple::point(0.0, 1.5, -5.0);
    let to = Tuple::point(0.0, 1.0, 0.0);
    let up = Tuple::vector(0.0, 1.0, 0.0);
    c.transform = view_transform(from, to, up);

    let image = render(Arc::new(c), Arc::new(w));
    image.to_png_file("world.png");
}
