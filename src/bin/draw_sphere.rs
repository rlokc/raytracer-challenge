use raytracer::canvas::Canvas;
use raytracer::colors::Color;
use raytracer::intersection::intersect;
use raytracer::light::{lighting, PointLight};
use raytracer::material::Material;
use raytracer::ray::Ray;
use raytracer::sphere::sphere;
use raytracer::tuple::Tuple;

pub fn main() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0f32;

    let canvas_pixels = 250;
    let pixel_size = wall_size / canvas_pixels as f32;

    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    // Set up the sphere
    let shape = sphere();
    let mut material = Material::default();
    material.color = Color::new(1.0, 0.2, 1.0);
    material.specular = 0.1;
    shape.lock().unwrap().set_material(&material);

    // Add a transformation
    // let transform = shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    // shape.lock().unwrap().set_transformation(&transform);

    // Set up the light
    let light_position = Tuple::point(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    println!("Drawing");
    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f32;

            let position = Tuple::point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, position.sub(ray_origin).normalize());
            let xs = intersect(shape.clone(), ray);

            match xs.hit() {
                None => (),
                Some(hit) => {
                    let point = ray.position(hit.t);
                    let normal = hit.scene_object.lock().unwrap().normal_at(point);
                    let eye = ray.direction.scalar_mul(-1.0);

                    let material = hit.scene_object.lock().unwrap().material();

                    let color = lighting(&material, &light, point, eye, normal);

                    canvas.write_pixel(x, y, color);
                }
            }
        }
    }

    println!("Flushing image to file");
    canvas.to_png_file("sphere.png");
}
