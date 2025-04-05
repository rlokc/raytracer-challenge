use std::f32::consts::PI;
use std::sync::Arc;
use raytracer::camera::Camera;
use raytracer::render::render;
use raytracer::transformations::view_transform;
use raytracer::tuple::Tuple;
use raytracer::world::World;

pub fn main() {
    let w = World::default_world();
    let mut c = Camera::new(640, 360, PI/2.0);
    let from = Tuple::point(0.0, 0.0, -5.0);
    let to = Tuple::point(0.0, 0.0, 0.0);
    let up = Tuple::vector(0.0, 1.0, 0.0);
    c.transform = view_transform(from, to, up);

    let image = render(Arc::new(c), Arc::new(w));
    image.to_png_file("render.png");
}