use raytracer::canvas::Canvas;
use raytracer::colors::Color;
use raytracer::intersection::intersect;
use raytracer::ray::Ray;
use raytracer::sphere::{sphere};
use raytracer::tuple::Tuple;

pub fn main() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0f32;

    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f32;

    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let shape = sphere();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f32;

            let position = Tuple::point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, position.sub(ray_origin).normalize());
            let xs = intersect(shape.clone(), r);

            match xs.hit() {
                None => (),
                Some(_) => {
                    canvas.write_pixel(x, y, color);
                },
            }
        }
    }

    canvas.to_ppm_file("sphere.ppm").unwrap();
}