use std::f32::consts::PI;

use raytracer::{canvas::Canvas, colors::Color, matrix::Matrix, tuple::Tuple};

const CANVAS_SIZE: usize = 400;

pub fn main() {

    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);

    let white = Color::new(1.0, 1.0, 1.0);

    for i in 0..12 {
        let p = Tuple::point(0.0, 0.0, 0.0);
        let transform = Matrix::identity_matrix(4)
            .translate(150.0, 0.0, 0.0)
            .rotate_z(-i as f32 * PI / 6.0)
            .translate(200.0, 200.0, 0.0);

        let point = transform.tuple_mul(&p);
        canvas.write_pixel(point.x.round() as usize, point.y.round() as usize, white);
    }

    canvas.to_ppm_file("clock.ppm").unwrap();
}